//! Ackee Bootcamp CTF — Level 3: Hidden Faction.
//!
//! Join the faction, obtain its token, and reveal the faction secret.
//! The evaluation mirrors the upstream flow: `show_faction_secret`
//! succeeds only for a member holding exactly one faction token AND the
//! correct secret preimage (the challenge's answer, player-supplied).

use crate::solana::SolChain;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use std::sync::Arc;

pub const PROGRAM_ID: &str = "5zjbNpnsSkCNG6zHzK183ujm6dn6fWeHWeUnk1Rzrs1Y";

const TOKEN_2022: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";
const ATA_PROGRAM: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";

/// Canonical system program (base58 "1111…1" = all-zero bytes).
fn system_id() -> Pubkey {
    Pubkey::new_from_array([0u8; 32])
}

fn token_2022_id() -> Pubkey {
    TOKEN_2022.parse().unwrap()
}

/// Anchor 8-byte discriminator from the level's shipped IDL (verified
/// equal to sha256("global:<name>")[..8]).
fn discriminator(name: &str) -> [u8; 8] {
    match name {
        "initialize" => [175, 175, 109, 31, 13, 152, 155, 237],
        "obtain_faction_token" => [85, 134, 20, 48, 83, 187, 45, 162],
        "show_faction_secret" => [211, 66, 73, 2, 244, 189, 57, 188],
        _ => panic!("unknown instruction {name}"),
    }
}

fn faction_pda(program: &Pubkey, authority: &Pubkey, mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[b"faction", authority.as_ref(), mint.as_ref()],
        program,
    )
    .0
}

/// Associated token account (Token-2022 flavor) for (owner, mint).
fn member_ata(owner: &Pubkey, mint: &Pubkey) -> Pubkey {
    let ata_program: Pubkey = ATA_PROGRAM.parse().unwrap();
    Pubkey::find_program_address(
        &[owner.as_ref(), token_2022_id().as_ref(), mint.as_ref()],
        &ata_program,
    )
    .0
}

#[derive(Clone)]
pub struct Target {
    pub program_id: Pubkey,
    /// The faction authority (also the evaluation member).
    pub authority: Arc<Keypair>,
    pub faction: Pubkey,
    pub mint: Pubkey,
}

impl Target {
    pub fn set_up(
        chain: &Arc<dyn SolChain>,
        roles: &crate::solana::SolRoles,
    ) -> eyre::Result<Self> {
        let program_id = PROGRAM_ID.parse::<Pubkey>()?;
        let so = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../lib/solana-auditors-bootcamp/Capture-the-Flag/level-3/target/deploy/level_3.so"
        );
        chain.deploy_program_from_file(program_id, so)?;

        // The offender founds the faction and is also the evaluation
        // member (they must obtain a token + the secret to solve).
        let authority = Arc::new(Keypair::new_from_array(
            roles.offender.keypair.to_bytes()[..32]
                .try_into()
                .expect("secret key bytes"),
        ));
        let mint = Keypair::new();
        chain.airdrop(&authority.pubkey(), 5_000_000_000)?;

        let faction =
            faction_pda(&program_id, &authority.pubkey(), &mint.pubkey());

        // initialize(name, symbol) — borsh strings.
        let (name, symbol) = ("ACKEE", "ACK");
        let mut data = discriminator("initialize").to_vec();
        data.extend_from_slice(&(name.len() as u32).to_le_bytes());
        data.extend_from_slice(name.as_bytes());
        data.extend_from_slice(&(symbol.len() as u32).to_le_bytes());
        data.extend_from_slice(symbol.as_bytes());

        let init_ix = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(authority.pubkey(), true),
                AccountMeta::new(faction, false),
                AccountMeta::new(mint.pubkey(), true),
                AccountMeta::new_readonly(system_id(), false),
                AccountMeta::new_readonly(token_2022_id(), false),
            ],
        );
        // The runtime's bundled spl-token-2022 writes larger extension
        // data than the pinned anchor allocated rent for; top the mint up
        // in the same transaction so the post-exec rent check passes.
        let topup_ix = solana_system_interface::instruction::transfer(
            &authority.pubkey(),
            &mint.pubkey(),
            3_000_000_000,
        );
        // The mint keypair co-signs its own creation; both signers in one
        // tx via send_signed.
        chain.send_signed(&[&authority, &mint], &[init_ix, topup_ix])?;

        Ok(Self { program_id, authority, faction, mint: mint.pubkey() })
    }

    /// Evaluation: `show_faction_secret` succeeds only when the member
    /// holds exactly one faction token AND supplied the correct secret.
    /// Failing tx (e.g. missing token or wrong secret) = unsolved.
    pub fn check(&self, chain: &Arc<dyn SolChain>) -> bool {
        let member = self.authority.pubkey();
        let ata = member_ata(&member, &self.mint);
        let ix = Instruction::new_with_bytes(
            self.program_id,
            &discriminator("show_faction_secret"),
            vec![
                AccountMeta::new_readonly(member, true),
                AccountMeta::new_readonly(self.faction, false),
                AccountMeta::new(ata, false),
                AccountMeta::new(self.mint, false),
            ],
        );
        chain.send_signed(&[&self.authority], &[ix]).is_ok()
    }
}
