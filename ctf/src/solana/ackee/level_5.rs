//! Ackee Bootcamp CTF — Level 5: Guardian Escrow.
//!
//! A guardian-protected escrow holds tokens for the offender; the
//! exploit must reach the state where `obtain_secret(secret)` succeeds.
//! Evaluation submits the player's discovered secret — the program
//! alone judges it, and no answer material lives here.
//!
//! Setup deploys both the level program and its verification program
//! (canonical genesis address), then initializes a guardian set and
//! the escrow using classic SPL Token, whose encoding era matches the
//! pinned program's CPIs.

use crate::solana::{ackee::Keyfile, rpc};
use serde::{Deserialize, Serialize};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
};

pub const PROGRAM_ID: &str = "5p55Gzc2FwGGHGp6DmncJaw9gbhLRpzj6Uca8Rv1bRhd";

/// Verification program shipped with the challenge (genesis address
/// from Anchor.toml's test.validator genesis entry).
pub const VERIFICATION_PROGRAM_ID: &str =
    "BqK4ZHrKeJudWinT6SyQe76uLoighkr7uvmE4bHCrv1L";

const SO: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../lib/solana-auditors-bootcamp/Capture-the-Flag/level-5/target/deploy/level_5.so"
);
const VERIFICATION_SO: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../lib/solana-auditors-bootcamp/Capture-the-Flag/level-5/accounts/verification-program/verification_program.so"
);

const TOKEN_PROGRAM: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const ATA_PROGRAM: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";

/// Canonical system program (base58 "1111…1" = all-zero bytes).
fn system_id() -> Pubkey {
    Pubkey::new_from_array([0u8; 32])
}

fn rent_id() -> Pubkey {
    "SysvarRent111111111111111111111111111111111"
        .parse()
        .expect("hardcoded rent sysvar address is valid")
}

fn token_program_id() -> Pubkey {
    TOKEN_PROGRAM.parse().expect("hardcoded SPL Token address is valid")
}

/// Anchor 8-byte instruction discriminators from the shipped IDL.
fn discriminator(name: &str) -> [u8; 8] {
    match name {
        "initialize" => [175, 175, 109, 31, 13, 152, 155, 237],
        "obtain_secret" => [123, 215, 246, 168, 114, 9, 183, 93],
        _ => panic!("unknown instruction {name}"),
    }
}

fn escrow_pda(
    program: &Pubkey,
    hacker: &Pubkey,
    sender: &Pubkey,
    escrow_token: &Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        &[b"escrow", hacker.as_ref(), sender.as_ref(), escrow_token.as_ref()],
        program,
    )
    .0
}

/// Classic SPL Token associated token account for `(owner, mint)`.
fn hacker_ata(owner: &Pubkey, mint: &Pubkey) -> Pubkey {
    let ata_program: Pubkey =
        ATA_PROGRAM.parse().expect("hardcoded ATA address is valid");
    Pubkey::find_program_address(
        &[owner.as_ref(), token_program_id().as_ref(), mint.as_ref()],
        &ata_program,
    )
    .0
}

/// Bridge a `solana_sdk` pubkey into the spl-token crate's own pubkey
/// type (it compiles against a newer Solana SDK generation).
fn bridge(pk: &Pubkey) -> spl_token::solana_program::pubkey::Pubkey {
    spl_token::solana_program::pubkey::Pubkey::new_from_array(pk.to_bytes())
}

async fn create_mint(
    rpc: &RpcClient,
    authority: &Keypair,
    mint: &Keypair,
    decimals: u8,
) -> eyre::Result<()> {
    let create = solana_system_interface::instruction::create_account(
        &authority.pubkey(),
        &mint.pubkey(),
        1_000_000_000,
        82,
        &token_program_id(),
    );
    let initialize = spl_token::instruction::initialize_mint(
        &bridge(&token_program_id()),
        &bridge(&mint.pubkey()),
        &bridge(&authority.pubkey()),
        None,
        decimals,
    )?;
    let initialize = Instruction::new_with_bytes(
        token_program_id(),
        &initialize.data,
        vec![
            AccountMeta::new(mint.pubkey(), false),
            AccountMeta::new_readonly(rent_id(), false),
        ],
    );
    rpc::send_instructions(rpc, authority, &[create, initialize], &[mint])
        .await
        .map(|_| ())
}

async fn ensure_ata(
    rpc: &RpcClient,
    owner: &Pubkey,
    payer: &Keypair,
    mint: &Pubkey,
) -> eyre::Result<Pubkey> {
    let ata = hacker_ata(owner, mint);
    if rpc
        .get_account_with_commitment(&ata, rpc.commitment())
        .await?
        .value
        .is_some()
    {
        return Ok(ata);
    }
    let create = Instruction::new_with_bytes(
        ATA_PROGRAM.parse().expect("hardcoded ATA address is valid"),
        &[1u8], // Create
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(ata, false),
            AccountMeta::new_readonly(*owner, false),
            AccountMeta::new_readonly(*mint, false),
            AccountMeta::new_readonly(system_id(), false),
            AccountMeta::new_readonly(token_program_id(), false),
            AccountMeta::new_readonly(rent_id(), false),
            AccountMeta::new_readonly(
                ATA_PROGRAM
                    .parse::<Pubkey>()
                    .expect("hardcoded ATA address is valid"),
                false,
            ),
        ],
    );
    rpc::send_instructions(rpc, payer, &[create], &[]).await.map(|_| ata)
}

async fn mint_to(
    rpc: &RpcClient,
    authority: &Keypair,
    mint: &Pubkey,
    target: &Pubkey,
    amount: u64,
) -> eyre::Result<()> {
    let instruction = spl_token::instruction::mint_to(
        &bridge(&token_program_id()),
        &bridge(mint),
        &bridge(target),
        &bridge(&authority.pubkey()),
        &[],
        amount,
    )?;
    let instruction = Instruction::new_with_bytes(
        token_program_id(),
        &instruction.data,
        vec![
            AccountMeta::new(*mint, false),
            AccountMeta::new(*target, false),
            AccountMeta::new_readonly(authority.pubkey(), true),
        ],
    );
    rpc::send_instructions(rpc, authority, &[instruction], &[])
        .await
        .map(|_| ())
}

/// Persistent setup identities for the guardian escrow level.
#[derive(Serialize, Deserialize)]
pub struct Actors {
    /// Funded sender that pays setup and deposits into the escrow.
    pub sender: Keyfile,
    /// The classic SPL Token mint created at setup.
    pub mint: Keyfile,
    /// The escrow's token account keypair; co-signs its creation.
    pub escrow_token: Keyfile,
}

impl Actors {
    pub(super) fn generate() -> Self {
        Self {
            sender: Keyfile::generate(),
            mint: Keyfile::generate(),
            escrow_token: Keyfile::generate(),
        }
    }
}

#[derive(Clone)]
pub struct Target {
    pub program_id: Pubkey,
    /// The verification program the escrow initializes through CPI.
    pub verification_program: Pubkey,
    /// The funded sender from setup.
    pub sender: Pubkey,
    /// The escrow account PDA.
    pub escrow: Pubkey,
    /// The escrow's token account.
    pub escrow_token: Pubkey,
    /// The escrow mint.
    pub mint: Pubkey,
}

impl Target {
    /// Deploy both programs and initialize the guardian escrow once on
    /// the persistent chain. The offender is the escrow recipient.
    pub async fn set_up(
        rpc: &RpcClient,
        actors: &Actors,
        offender: &Keypair,
    ) -> eyre::Result<()> {
        rpc::write_program_if_absent(rpc, PROGRAM_ID, SO).await?;
        rpc::write_program_if_absent(
            rpc,
            VERIFICATION_PROGRAM_ID,
            VERIFICATION_SO,
        )
        .await?;
        let sender = actors.sender.keypair()?;
        let mint = actors.mint.keypair()?;
        let escrow_token = actors.escrow_token.keypair()?;
        let program_id = PROGRAM_ID.parse()?;
        let escrow = escrow_pda(
            &program_id,
            &offender.pubkey(),
            &sender.pubkey(),
            &escrow_token.pubkey(),
        );
        if super::account_owned_by(rpc, &escrow, &program_id).await? {
            return Ok(());
        }
        rpc::fund_at_least(rpc, &sender.pubkey(), 5_000_000_000).await?;

        create_mint(rpc, &sender, &mint, 9).await?;
        let sender_ata =
            ensure_ata(rpc, &sender.pubkey(), &sender, &mint.pubkey()).await?;
        ensure_ata(rpc, &offender.pubkey(), &sender, &mint.pubkey()).await?;
        mint_to(rpc, &sender, &mint.pubkey(), &sender_ata, 1_000_000).await?;

        let verification_id: Pubkey = VERIFICATION_PROGRAM_ID.parse()?;
        let guardian_set_index: u32 = 0;
        let escrow_pda_authority = Pubkey::find_program_address(
            &[b"ESCROW_PDA_AUTHORITY"],
            &program_id,
        )
        .0;

        // initialize(guardian_set_index u32, signatures_number u8,
        // expiration_time i64, signatures Vec<u64>, amount_in u64). The
        // verification program requires at least four guardian keys.
        let signatures_number: u8 = 4;
        let expiration_time: i64 = 4_000_000_000;
        let signatures: Vec<u64> = vec![1, 2, 3, 4];
        let amount_in: u64 = 100_000;
        let mut data = discriminator("initialize").to_vec();
        data.extend_from_slice(&guardian_set_index.to_le_bytes());
        data.push(signatures_number);
        data.extend_from_slice(&expiration_time.to_le_bytes());
        data.extend_from_slice(&(signatures.len() as u32).to_le_bytes());
        for signature in &signatures {
            data.extend_from_slice(&signature.to_le_bytes());
        }
        data.extend_from_slice(&amount_in.to_le_bytes());

        let initialize = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(sender.pubkey(), true),
                AccountMeta::new_readonly(offender.pubkey(), false),
                AccountMeta::new(
                    Pubkey::find_program_address(
                        &[
                            b"guardian_set",
                            offender.pubkey().as_ref(),
                            &guardian_set_index.to_be_bytes(),
                        ],
                        &verification_id,
                    )
                    .0,
                    false,
                ),
                AccountMeta::new(escrow, false),
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
        rpc::send_instructions(rpc, &sender, &[initialize], &[&escrow_token])
            .await?;
        Ok(())
    }

    /// Load the level from the persistent chain.
    pub async fn from_rpc(
        rpc: &RpcClient,
        actors: &Actors,
        offender_pubkey: &Pubkey,
    ) -> eyre::Result<Self> {
        let program_id = super::require_program(
            rpc,
            PROGRAM_ID,
            "level 5 (Guardian Escrow)",
        )
        .await?;
        super::require_program(
            rpc,
            VERIFICATION_PROGRAM_ID,
            "level 5 verification program",
        )
        .await?;
        let sender = actors.sender.keypair()?.pubkey();
        let escrow_token = actors.escrow_token.keypair()?.pubkey();
        let escrow =
            escrow_pda(&program_id, offender_pubkey, &sender, &escrow_token);
        eyre::ensure!(
            super::account_owned_by(rpc, &escrow, &program_id).await?,
            "Guardian Escrow is not initialized; rerun cargo run -p ctf --bin deploy_levels -- solana and check its level 5 output"
        );
        Ok(Self {
            program_id,
            verification_program: VERIFICATION_PROGRAM_ID.parse()?,
            sender,
            escrow,
            escrow_token,
            mint: actors.mint.keypair()?.pubkey(),
        })
    }

    /// Evaluation: submit the player's discovered secret to
    /// `obtain_secret` as the hacker. `None` (no answer saved yet)
    /// reports unsolved without a transaction; a wrong secret fails
    /// on-chain.
    pub async fn check_rpc(
        &self,
        rpc: &RpcClient,
        hacker: &Keypair,
        secret: Option<&str>,
    ) -> eyre::Result<bool> {
        let Some(secret) = secret else {
            return Ok(false);
        };
        let mut data = discriminator("obtain_secret").to_vec();
        data.extend_from_slice(&(secret.len() as u32).to_le_bytes());
        data.extend_from_slice(secret.as_bytes());
        let instruction = Instruction::new_with_bytes(
            self.program_id,
            &data,
            vec![
                AccountMeta::new_readonly(hacker.pubkey(), true),
                AccountMeta::new(
                    hacker_ata(&hacker.pubkey(), &self.mint),
                    false,
                ),
                AccountMeta::new(self.mint, false),
            ],
        );
        Ok(rpc::send_instructions(rpc, hacker, &[instruction], &[])
            .await
            .is_ok())
    }
}
