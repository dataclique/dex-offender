//! Ackee Bootcamp CTF — Level 3: Hidden Faction.
//!
//! Join the faction, obtain exactly one faction token, and reveal the
//! faction secret. Setup initializes the faction and its Token-2022
//! mint under a persistent creator identity; `obtain_faction_token`
//! requires that faction authority's co-signature, so the attack
//! receives it from the target. Evaluation submits the player's
//! discovered secret to `show_faction_secret` — the program alone
//! judges it, and no answer material lives here.

use crate::solana::{ackee::Keyfile, rpc};
use serde::{Deserialize, Serialize};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
};
use std::sync::Arc;

pub const PROGRAM_ID: &str = "5zjbNpnsSkCNG6zHzK183ujm6dn6fWeHWeUnk1Rzrs1Y";

const SO: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../lib/solana-auditors-bootcamp/Capture-the-Flag/level-3/target/deploy/level_3.so"
);

const TOKEN_2022: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";
const ATA_PROGRAM: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";

/// Canonical system program (base58 "1111…1" = all-zero bytes).
fn system_id() -> Pubkey {
    Pubkey::new_from_array([0u8; 32])
}

fn token_2022_id() -> Pubkey {
    TOKEN_2022.parse().expect("hardcoded Token-2022 address is valid")
}

/// Anchor 8-byte instruction discriminator: sha256("global:<name>")[..8].
fn discriminator(name: &str) -> [u8; 8] {
    let hash = solana_sdk::hash::hash(format!("global:{name}").as_bytes());
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&hash.as_ref()[..8]);
    discriminator
}

fn faction_pda(program: &Pubkey, authority: &Pubkey, mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[b"faction", authority.as_ref(), mint.as_ref()],
        program,
    )
    .0
}

/// Token-2022 associated token account for `(owner, mint)`.
fn member_ata(owner: &Pubkey, mint: &Pubkey) -> Pubkey {
    let ata_program: Pubkey =
        ATA_PROGRAM.parse().expect("hardcoded ATA address is valid");
    Pubkey::find_program_address(
        &[owner.as_ref(), token_2022_id().as_ref(), mint.as_ref()],
        &ata_program,
    )
    .0
}

/// Persistent setup identities: the faction creator/authority (its
/// co-signature authorizes joins) and the mint keypair that co-signs
/// `initialize`.
#[derive(Serialize, Deserialize)]
pub struct Actors {
    /// Faction creator and authority; co-signs `obtain_faction_token`.
    pub creator: Keyfile,
    /// The faction mint, created by `initialize` and co-signed here.
    pub mint: Keyfile,
}

impl Actors {
    pub(super) fn generate() -> Self {
        Self { creator: Keyfile::generate(), mint: Keyfile::generate() }
    }
}

#[derive(Clone)]
pub struct Target {
    pub program_id: Pubkey,
    /// Faction authority; co-signs joins during the exploit.
    pub faction_authority: Arc<Keypair>,
    /// The faction account PDA.
    pub faction: Pubkey,
    /// The faction mint created at setup.
    pub mint: Pubkey,
}

impl Target {
    /// Deploy the program and initialize the faction once on the
    /// persistent chain.
    pub async fn set_up(
        rpc: &RpcClient,
        actors: &Actors,
        _offender: &Keypair,
    ) -> eyre::Result<()> {
        rpc::write_program_if_absent(rpc, PROGRAM_ID, SO).await?;
        let creator = actors.creator.keypair()?;
        let mint = actors.mint.keypair()?;
        let program_id = PROGRAM_ID.parse()?;
        let faction =
            faction_pda(&program_id, &creator.pubkey(), &mint.pubkey());
        if super::account_owned_by(rpc, &faction, &program_id).await? {
            return Ok(());
        }
        rpc::fund_at_least(rpc, &creator.pubkey(), 5_000_000_000).await?;

        // initialize(name, symbol) — borsh strings.
        let (name, symbol) = ("ACKEE", "ACK");
        let mut data = discriminator("initialize").to_vec();
        data.extend_from_slice(&(name.len() as u32).to_le_bytes());
        data.extend_from_slice(name.as_bytes());
        data.extend_from_slice(&(symbol.len() as u32).to_le_bytes());
        data.extend_from_slice(symbol.as_bytes());
        let initialize = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(creator.pubkey(), true),
                AccountMeta::new(faction, false),
                AccountMeta::new(mint.pubkey(), true),
                AccountMeta::new_readonly(system_id(), false),
                AccountMeta::new_readonly(token_2022_id(), false),
            ],
        );
        // The runtime's Token-2022 writes larger extension data than
        // the pinned program allocated rent for; top the mint up in
        // the same transaction so the post-execution rent check passes.
        let topup = solana_system_interface::instruction::transfer(
            &creator.pubkey(),
            &mint.pubkey(),
            3_000_000_000,
        );
        rpc::send_instructions(rpc, &creator, &[initialize, topup], &[&mint])
            .await?;
        Ok(())
    }

    /// Load the level from the persistent chain.
    pub async fn from_rpc(
        rpc: &RpcClient,
        actors: &Actors,
    ) -> eyre::Result<Self> {
        let program_id =
            super::require_program(rpc, PROGRAM_ID, "level 3 (Hidden Faction)")
                .await?;
        let creator = Arc::new(actors.creator.keypair()?);
        let mint = actors.mint.keypair()?;
        let faction =
            faction_pda(&program_id, &creator.pubkey(), &mint.pubkey());
        eyre::ensure!(
            super::account_owned_by(rpc, &faction, &program_id).await?,
            "Hidden Faction is not initialized; rerun cargo run -p ctf --bin deploy_levels -- solana"
        );
        Ok(Self {
            program_id,
            faction_authority: creator,
            faction,
            mint: mint.pubkey(),
        })
    }

    /// Evaluation: submit the player's discovered secret to
    /// `show_faction_secret` as the member. `None` (no answer saved
    /// yet) reports unsolved without a transaction; a wrong secret
    /// fails on-chain.
    pub async fn check_rpc(
        &self,
        rpc: &RpcClient,
        member: &Keypair,
        secret: Option<&str>,
    ) -> eyre::Result<bool> {
        let Some(secret) = secret else {
            return Ok(false);
        };
        let mut data = discriminator("show_faction_secret").to_vec();
        data.extend_from_slice(&(secret.len() as u32).to_le_bytes());
        data.extend_from_slice(secret.as_bytes());
        let instruction = Instruction::new_with_bytes(
            self.program_id,
            &data,
            vec![
                AccountMeta::new_readonly(member.pubkey(), true),
                AccountMeta::new_readonly(self.faction, false),
                AccountMeta::new(
                    member_ata(&member.pubkey(), &self.mint),
                    false,
                ),
                AccountMeta::new(self.mint, false),
            ],
        );
        Ok(rpc::send_instructions(rpc, member, &[instruction], &[])
            .await
            .is_ok())
    }
}
