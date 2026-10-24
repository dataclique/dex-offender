//! Ackee Bootcamp CTF — Level 5: Guardian Escrow.
//!
//! Initialize a guardian-protected escrow, then obtain the secret. The
//! evaluation mirrors the upstream test: `obtain_secret(secret)`
//! succeeds only with the correct secret (pinned inside the `.so` —
//! player-supplied, never embedded here). Uses the CLASSIC spl-token
//! program so the pinned anchor-0.30 CPIs stay in the compatible
//! encoding era.

use crate::solana::SolChain;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use std::sync::Arc;

pub const PROGRAM_ID: &str = "5p55Gzc2FwGGHGp6DmncJaw9gbhLRpzj6Uca8Rv1bRhd";

/// Verification program shipped with the challenge (genesis address
/// from Anchor.toml's test.validator genesis entry).
pub const VERIFICATION_PROGRAM_ID: &str =
    "BqK4ZHrKeJudWinT6SyQe76uLoighkr7uvmE4bHCrv1L";

const TOKEN_PROGRAM: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const ATA_PROGRAM: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";

/// Canonical system program (base58 "1111…1" = all-zero bytes).
fn system_id() -> Pubkey {
    Pubkey::new_from_array([0u8; 32])
}

fn rent_id() -> Pubkey {
    "SysvarRent111111111111111111111111111111111".parse().unwrap()
}

fn token_program_id() -> Pubkey {
    TOKEN_PROGRAM.parse().unwrap()
}

fn discriminator(name: &str) -> [u8; 8] {
    match name {
        "initialize" => [175, 175, 109, 31, 13, 152, 155, 237],
        "obtain_secret" => [123, 215, 246, 168, 114, 9, 183, 93],
        _ => panic!("unknown instruction {name}"),
    }
}

fn associated_token_address(owner: &Pubkey, mint: &Pubkey) -> Pubkey {
    let ata_program: Pubkey = ATA_PROGRAM.parse().unwrap();
    Pubkey::find_program_address(
        &[owner.as_ref(), token_program_id().as_ref(), mint.as_ref()],
        &ata_program,
    )
    .0
}

fn create_mint(
    chain: &Arc<dyn SolChain>,
    authority: &Keypair,
    mint: &Keypair,
    decimals: u8,
) -> eyre::Result<()> {
    let create_ixn = solana_system_interface::instruction::create_account(
        &authority.pubkey(),
        &mint.pubkey(),
        1_000_000_000,
        82,
        &token_program_id(),
    );
    let bridge = |pk: &Pubkey| {
        spl_token::solana_program::pubkey::Pubkey::new_from_array(pk.to_bytes())
    };
    let init_theirs = spl_token::instruction::initialize_mint(
        &bridge(&token_program_id()),
        &bridge(&mint.pubkey()),
        &bridge(&authority.pubkey()),
        None,
        decimals,
    )?;
    let init_ixn = Instruction::new_with_bytes(
        token_program_id(),
        &init_theirs.data,
        vec![
            AccountMeta::new(mint.pubkey(), false),
            AccountMeta::new_readonly(rent_id(), false),
        ],
    );
    chain.send_signed(&[authority, mint], &[create_ixn, init_ixn])?;
    Ok(())
}

fn ensure_ata(
    chain: &Arc<dyn SolChain>,
    owner: &Pubkey,
    payer: &Keypair,
    mint: &Pubkey,
) -> eyre::Result<Pubkey> {
    let ata = associated_token_address(owner, mint);
    if chain.get_account(&ata).is_some() {
        return Ok(ata);
    }
    let ixn = Instruction::new_with_bytes(
        ATA_PROGRAM.parse()?,
        &[1u8], // Create
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(ata, false),
            AccountMeta::new_readonly(*owner, false),
            AccountMeta::new_readonly(*mint, false),
            AccountMeta::new_readonly(system_id(), false),
            AccountMeta::new_readonly(token_program_id(), false),
            AccountMeta::new_readonly(rent_id(), false),
            AccountMeta::new_readonly(ATA_PROGRAM.parse::<Pubkey>()?, false),
        ],
    );
    chain.send(payer, &[ixn])?;
    Ok(ata)
}

fn mint_to(
    chain: &Arc<dyn SolChain>,
    authority: &Keypair,
    mint: &Pubkey,
    target: &Pubkey,
    amount: u64,
) -> eyre::Result<()> {
    let bridge = |pk: &Pubkey| {
        spl_token::solana_program::pubkey::Pubkey::new_from_array(pk.to_bytes())
    };
    let ixn_theirs = spl_token::instruction::mint_to(
        &bridge(&token_program_id()),
        &bridge(mint),
        &bridge(target),
        &bridge(&authority.pubkey()),
        &[],
        amount,
    )?;
    let ixn = Instruction::new_with_bytes(
        token_program_id(),
        &ixn_theirs.data,
        vec![
            AccountMeta::new(*mint, false),
            AccountMeta::new(*target, false),
            AccountMeta::new_readonly(authority.pubkey(), true),
        ],
    );
    chain.send(authority, &[ixn])?;
    Ok(())
}

#[derive(Clone)]
pub struct Target {
    pub program_id: Pubkey,
    /// The vesting recipient / evaluation caller (offender).
    pub hacker: Arc<Keypair>,
    pub mint: Pubkey,
    pub hacker_ata: Pubkey,
}

impl Target {
    pub fn set_up(
        chain: &Arc<dyn SolChain>,
        roles: &crate::solana::SolRoles,
    ) -> eyre::Result<Self> {
        let program_id = PROGRAM_ID.parse::<Pubkey>()?;
        let so = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../lib/solana-auditors-bootcamp/Capture-the-Flag/level-5/target/deploy/level_5.so"
        );
        chain.deploy_program_from_file(program_id, so)?;
        let verification_id: Pubkey = VERIFICATION_PROGRAM_ID.parse()?;
        let verification_so = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../lib/solana-auditors-bootcamp/Capture-the-Flag/level-5/accounts/verification-program/verification_program.so"
        );
        chain.deploy_program_from_file(verification_id, verification_so)?;

        // Fresh state mirroring the upstream setup: a funded sender
        // initializes a guardian escrow vesting tokens to the offender.
        let hacker = Arc::new(Keypair::new_from_array(
            roles.offender.keypair.to_bytes()[..32]
                .try_into()
                .expect("secret key bytes"),
        ));
        let sender = Keypair::new();
        let mint = Keypair::new();
        chain.airdrop(&sender.pubkey(), 5_000_000_000)?;
        chain.airdrop(&hacker.pubkey(), 2_000_000_000)?;

        create_mint(chain, &sender, &mint, 6)?;
        let sender_ata =
            ensure_ata(chain, &sender.pubkey(), &sender, &mint.pubkey())?;
        let hacker_ata =
            ensure_ata(chain, &hacker.pubkey(), &sender, &mint.pubkey())?;
        mint_to(chain, &sender, &mint.pubkey(), &sender_ata, 1_000_000)?;

        // The escrow token account is program-created (init) with the
        // escrow PDA authority — pass a fresh keypair it co-signs.
        let escrow_token = Keypair::new();
        let escrow_pda_authority = Pubkey::find_program_address(
            &[b"ESCROW_PDA_AUTHORITY"],
            &program_id,
        )
        .0;

        // initialize(guardian_set_index u32, signatures_number u8,
        // expiration_time i64, signatures Vec<u64>, amount_in u64).
        let guardian_set_index: u32 = 0;
        let signatures_number: u8 = 4;
        let expiration_time: i64 = 4_000_000_000;
        // The verification program requires >= 4 guardian keys.
        let signatures: Vec<u64> = vec![1, 2, 3, 4];
        let amount_in: u64 = 100_000;
        let mut data = discriminator("initialize").to_vec();
        data.extend_from_slice(&guardian_set_index.to_le_bytes());
        data.push(signatures_number);
        data.extend_from_slice(&expiration_time.to_le_bytes());
        data.extend_from_slice(&(signatures.len() as u32).to_le_bytes());
        for s in &signatures {
            data.extend_from_slice(&s.to_le_bytes());
        }
        data.extend_from_slice(&amount_in.to_le_bytes());

        let ix = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(sender.pubkey(), true),
                AccountMeta::new_readonly(hacker.pubkey(), false),
                AccountMeta::new(
                    Pubkey::find_program_address(
                        &[
                            b"guardian_set",
                            hacker.pubkey().as_ref(),
                            &guardian_set_index.to_be_bytes(),
                        ],
                        &verification_id,
                    )
                    .0,
                    false,
                ),
                AccountMeta::new(
                    Pubkey::find_program_address(
                        &[
                            b"escrow",
                            hacker.pubkey().as_ref(),
                            sender.pubkey().as_ref(),
                            escrow_token.pubkey().as_ref(),
                        ],
                        &program_id,
                    )
                    .0,
                    false,
                ),
                AccountMeta::new(escrow_token.pubkey(), true),
                AccountMeta::new_readonly(escrow_pda_authority, false),
                AccountMeta::new(sender_ata, false),
                AccountMeta::new_readonly(mint.pubkey(), false),
                AccountMeta::new_readonly(token_program_id(), false),
                AccountMeta::new_readonly(system_id(), false),
                AccountMeta::new_readonly(verification_id, false),
            ],
        );
        // The escrow token account keypair co-signs its creation.
        if let Err(e) = chain.send_signed(&[&sender, &escrow_token], &[ix]) {
            println!("initialize CPI status: {e}");
        }

        Ok(Self { program_id, hacker, mint: mint.pubkey(), hacker_ata })
    }

    /// Evaluation: `obtain_secret` succeeds only with the correct secret
    /// (and the expected token state). Failing tx = unsolved.
    pub fn check(&self, chain: &Arc<dyn SolChain>) -> bool {
        // Well-formed call with a wrong secret: fails semantically until
        // the real secret (player-supplied) is used.
        let secret = "unsolved";
        let mut data = discriminator("obtain_secret").to_vec();
        data.extend_from_slice(&(secret.len() as u32).to_le_bytes());
        data.extend_from_slice(secret.as_bytes());
        let ix = Instruction::new_with_bytes(
            self.program_id,
            &data,
            vec![
                AccountMeta::new_readonly(self.hacker.pubkey(), true),
                AccountMeta::new(self.hacker_ata, false),
                AccountMeta::new(self.mint, false),
            ],
        );
        chain.send_signed(&[&self.hacker], &[ix]).is_ok()
    }
}
