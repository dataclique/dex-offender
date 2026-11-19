//! Ackee Bootcamp CTF — Level 4: Vesting.
//!
//! A vesting escrow holds tokens for the offender; the exploit must
//! reach the state where `reveal_secret(secret)` succeeds for the
//! holder. Evaluation submits the player's discovered secret — the
//! program alone judges it, and no answer material lives here.
//!
//! Setup mints a Token-2022 token, funds accounts, and runs
//! `init_vesting`. The pinned program's `transfer_checked` CPI dates
//! from the Anchor 0.30 / Token-2022 3.0 era: if the runtime's bundled
//! Token-2022 rejects that encoding, setup fails with the exact CPI
//! error and the level stays blocked (the runner reports it as
//! uncheckable instead of pretending it is playable).

use crate::solana::{ackee::Keyfile, rpc};
use serde::{Deserialize, Serialize};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
};

pub const PROGRAM_ID: &str = "D51vhx6jAbBtQVwo1fcYr7RMKQKAAnSUy6v7vRCHCZL3";

const SO: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../lib/solana-auditors-bootcamp/Capture-the-Flag/level-4/target/deploy/level_4.so"
);

const TOKEN_2022: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";
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

fn token_2022_id() -> Pubkey {
    TOKEN_2022.parse().expect("hardcoded Token-2022 address is valid")
}

/// Anchor 8-byte instruction discriminators from the shipped IDL.
fn discriminator(name: &str) -> [u8; 8] {
    match name {
        "init_vesting" => [119, 192, 67, 41, 47, 82, 152, 27],
        "reveal_secret" => [126, 156, 142, 60, 92, 135, 177, 144],
        _ => panic!("unknown instruction {name}"),
    }
}

/// Token-2022 associated token account for `(owner, mint)`.
fn hacker_ata(owner: &Pubkey, mint: &Pubkey) -> Pubkey {
    let ata_program: Pubkey =
        ATA_PROGRAM.parse().expect("hardcoded ATA address is valid");
    Pubkey::find_program_address(
        &[owner.as_ref(), token_2022_id().as_ref(), mint.as_ref()],
        &ata_program,
    )
    .0
}

/// Bridge a `solana_sdk` pubkey into the token-interface crates' own
/// address type (they compile against a newer Solana SDK generation).
fn bridge(pk: &Pubkey) -> solana_address::Address {
    solana_address::Address::new_from_array(pk.to_bytes())
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
        &token_2022_id(),
    );
    let initialize = spl_token_2022_interface::instruction::initialize_mint2(
        &bridge(&token_2022_id()),
        &bridge(&mint.pubkey()),
        &bridge(&authority.pubkey()),
        None,
        decimals,
    )?;
    let initialize = Instruction::new_with_bytes(
        token_2022_id(),
        &initialize.data,
        vec![AccountMeta::new(mint.pubkey(), false)],
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
            AccountMeta::new_readonly(token_2022_id(), false),
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
    let instruction = spl_token_2022_interface::instruction::mint_to(
        &bridge(&token_2022_id()),
        &bridge(mint),
        &bridge(target),
        &bridge(&authority.pubkey()),
        &[],
        amount,
    )?;
    let instruction = Instruction::new_with_bytes(
        token_2022_id(),
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

/// Persistent setup identities for the vesting level.
#[derive(Serialize, Deserialize)]
pub struct Actors {
    /// Funded sender that pays setup and initializes the escrow.
    pub sender: Keyfile,
    /// The Token-2022 mint created at setup.
    pub mint: Keyfile,
    /// The escrow account keypair; co-signs `init_vesting`.
    pub escrow: Keyfile,
    /// The escrow's token account keypair; co-signs its creation.
    pub escrow_token: Keyfile,
}

impl Actors {
    pub(super) fn generate() -> Self {
        Self {
            sender: Keyfile::generate(),
            mint: Keyfile::generate(),
            escrow: Keyfile::generate(),
            escrow_token: Keyfile::generate(),
        }
    }
}

#[derive(Clone)]
pub struct Target {
    pub program_id: Pubkey,
    /// The funded sender from setup.
    pub sender: Pubkey,
    /// The escrow account created by `init_vesting`.
    pub escrow: Pubkey,
    /// The escrow's token account.
    pub escrow_token: Pubkey,
    /// The vesting mint.
    pub mint: Pubkey,
}

impl Target {
    /// Deploy the program and initialize the vesting escrow once on
    /// the persistent chain. The offender is the vesting recipient.
    pub async fn set_up(
        rpc: &RpcClient,
        actors: &Actors,
        offender: &Keypair,
    ) -> eyre::Result<()> {
        rpc::write_program_if_absent(rpc, PROGRAM_ID, SO).await?;
        let sender = actors.sender.keypair()?;
        let mint = actors.mint.keypair()?;
        let escrow = actors.escrow.keypair()?;
        let escrow_token = actors.escrow_token.keypair()?;
        let program_id = PROGRAM_ID.parse()?;
        if super::account_owned_by(rpc, &escrow.pubkey(), &program_id).await? {
            return Ok(());
        }
        rpc::fund_at_least(rpc, &sender.pubkey(), 5_000_000_000).await?;

        create_mint(rpc, &sender, &mint, 6).await?;
        let sender_ata =
            ensure_ata(rpc, &sender.pubkey(), &sender, &mint.pubkey()).await?;
        ensure_ata(rpc, &offender.pubkey(), &sender, &mint.pubkey()).await?;
        mint_to(rpc, &sender, &mint.pubkey(), &sender_ata, 1_000_000).await?;

        // The escrow's token account starts owned by the sender (the
        // program reassigns its authority to its PDA during init).
        let create = solana_system_interface::instruction::create_account(
            &sender.pubkey(),
            &escrow_token.pubkey(),
            1_000_000_000,
            165,
            &token_2022_id(),
        );
        let initialize =
            spl_token_2022_interface::instruction::initialize_account3(
                &bridge(&token_2022_id()),
                &bridge(&escrow_token.pubkey()),
                &bridge(&mint.pubkey()),
                &bridge(&sender.pubkey()),
            )?;
        let initialize = Instruction::new_with_bytes(
            token_2022_id(),
            &initialize.data,
            initialize
                .accounts
                .iter()
                .map(|meta| AccountMeta {
                    pubkey: Pubkey::new_from_array(meta.pubkey.to_bytes()),
                    is_signer: meta.is_signer,
                    is_writable: meta.is_writable,
                })
                .collect::<Vec<_>>(),
        );
        rpc::send_instructions(
            rpc,
            &sender,
            &[create, initialize],
            &[&escrow_token],
        )
        .await?;

        // init_vesting(recipient, amount, start_at, end_at, interval).
        let mut data = discriminator("init_vesting").to_vec();
        data.extend_from_slice(offender.pubkey().as_ref());
        data.extend_from_slice(&1_000_000u64.to_le_bytes());
        data.extend_from_slice(&0u64.to_le_bytes()); // start_at
        data.extend_from_slice(&1_000_000u64.to_le_bytes()); // end_at
        data.extend_from_slice(&1u64.to_le_bytes()); // interval
        let init_vesting = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(sender.pubkey(), true),
                AccountMeta::new(sender_ata, false),
                AccountMeta::new(escrow.pubkey(), true),
                AccountMeta::new(escrow_token.pubkey(), false),
                AccountMeta::new_readonly(mint.pubkey(), false),
                AccountMeta::new_readonly(token_2022_id(), false),
                AccountMeta::new_readonly(system_id(), false),
            ],
        );
        rpc::send_instructions(rpc, &sender, &[init_vesting], &[&escrow])
            .await?;
        Ok(())
    }

    /// Load the level from the persistent chain.
    pub async fn from_rpc(
        rpc: &RpcClient,
        actors: &Actors,
    ) -> eyre::Result<Self> {
        let program_id =
            super::require_program(rpc, PROGRAM_ID, "level 4 (Vesting)")
                .await?;
        let escrow = actors.escrow.keypair()?.pubkey();
        eyre::ensure!(
            super::account_owned_by(rpc, &escrow, &program_id).await?,
            "Vesting escrow is not initialized; rerun cargo run -p ctf --bin deploy_levels -- solana and check its level 4 output"
        );
        Ok(Self {
            program_id,
            sender: actors.sender.keypair()?.pubkey(),
            escrow,
            escrow_token: actors.escrow_token.keypair()?.pubkey(),
            mint: actors.mint.keypair()?.pubkey(),
        })
    }

    /// Evaluation: submit the player's discovered secret to
    /// `reveal_secret` as the hacker. `None` (no answer saved yet)
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
        let mut data = discriminator("reveal_secret").to_vec();
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
