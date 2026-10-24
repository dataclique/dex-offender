//! Ackee Bootcamp CTF — Level 4: Vesting.
//!
//! Initialize a vesting escrow, then withdraw early. The evaluation
//! mirrors the upstream test: `reveal_secret(secret)` succeeds only for
//! the holder with the correct secret (hash pinned inside the `.so` —
//! player-supplied, never embedded here).
//!
//! SETUP STATUS: the pinned .so's internal `transfer_checked` CPI uses
//! the classic Token-2022 instruction encoding (anchor-spl 0.30 /
//! spl-token-2022 3.0.4-era), which the runtime's bundled
//! spl_token_2022-11.0.0 rejects (`InvalidInstructionData` under
//! wincode). Unblocking needs an era-matched Token-2022 binary deployed
//! over `TokenzQd…` (not shipped in the pinned repo) or an upstream
//! rebuild. Escrow/mint/account construction up to that CPI is verified
//! working below.

use crate::solana::SolChain;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use std::sync::Arc;

pub const PROGRAM_ID: &str = "D51vhx6jAbBtQVwo1fcYr7RMKQKAAnSUy6v7vRCHCZL3";

const TOKEN_PROGRAM: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";
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

/// Anchor 8-byte discriminators from the level's shipped IDL.
fn discriminator(name: &str) -> [u8; 8] {
    match name {
        "init_vesting" => [119, 192, 67, 41, 47, 82, 152, 27],
        "reveal_secret" => [126, 156, 142, 60, 92, 135, 177, 144],
        "withdraw_unlocked" => [213, 161, 76, 199, 38, 28, 209, 80],
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

/// Borrowed from the hana module's proven helpers (same encoding rules).
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
    // Token-2022 encoding via the interface crate (authoritative for
    // the runtime's wincode era), bridged across the version gap via
    // bytes.
    let bridge =
        |pk: &Pubkey| solana_address::Address::new_from_array(pk.to_bytes());
    let init_theirs = spl_token_2022_interface::instruction::initialize_mint2(
        &bridge(&token_program_id()),
        &bridge(&mint.pubkey()),
        &bridge(&authority.pubkey()),
        None,
        decimals,
    )?;
    let init_ixn = Instruction::new_with_bytes(
        token_program_id(),
        &init_theirs.data,
        vec![AccountMeta::new(mint.pubkey(), false)],
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
    let bridge =
        |pk: &Pubkey| solana_address::Address::new_from_array(pk.to_bytes());
    let ixn_theirs = spl_token_2022_interface::instruction::mint_to(
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
            "/../lib/solana-auditors-bootcamp/Capture-the-Flag/level-4/target/deploy/level_4.so"
        );
        chain.deploy_program_from_file(program_id, so)?;

        // Fresh state mirroring the upstream setup: a funded sender
        // initializes a vesting escrow for the offender (hacker).
        let hacker = Arc::new(Keypair::new_from_array(
            roles.offender.keypair.to_bytes()[..32]
                .try_into()
                .expect("secret key bytes"),
        ));
        let sender = Keypair::new();
        let escrow = Keypair::new();
        let mint = Keypair::new();
        chain.airdrop(&sender.pubkey(), 5_000_000_000)?;
        chain.airdrop(&hacker.pubkey(), 2_000_000_000)?;

        create_mint(chain, &sender, &mint, 6)?;
        let sender_ata =
            ensure_ata(chain, &sender.pubkey(), &sender, &mint.pubkey())?;
        let hacker_ata =
            ensure_ata(chain, &hacker.pubkey(), &sender, &mint.pubkey())?;
        mint_to(chain, &sender, &mint.pubkey(), &sender_ata, 1_000_000)?;

        // The escrow's token account starts owned by the SENDER (the
        // program reassigns its authority to its PDA during init).
        let escrow_token = Keypair::new();
        let create_escrow_acct =
            solana_system_interface::instruction::create_account(
                &sender.pubkey(),
                &escrow_token.pubkey(),
                1_000_000_000,
                165,
                &token_program_id(),
            );
        let bridge = |pk: &Pubkey| {
            solana_address::Address::new_from_array(pk.to_bytes())
        };
        let init_theirs =
            spl_token_2022_interface::instruction::initialize_account3(
                &bridge(&token_program_id()),
                &bridge(&escrow_token.pubkey()),
                &bridge(&mint.pubkey()),
                &bridge(&sender.pubkey()),
            )?;
        let init_escrow_acct = Instruction::new_with_bytes(
            token_program_id(),
            &init_theirs.data,
            init_theirs
                .accounts
                .iter()
                .map(|m| AccountMeta {
                    pubkey: Pubkey::new_from_array(m.pubkey.to_bytes()),
                    is_signer: m.is_signer,
                    is_writable: m.is_writable,
                })
                .collect::<Vec<_>>(),
        );
        chain.send_signed(
            &[&sender, &escrow_token],
            &[create_escrow_acct, init_escrow_acct],
        )?;

        // init_vesting(recipient, amount, start_at, end_at, interval).
        let now = 0u64;
        let mut data = discriminator("init_vesting").to_vec();
        data.extend_from_slice(hacker.pubkey().as_ref());
        data.extend_from_slice(&1_000_000u64.to_le_bytes());
        data.extend_from_slice(&now.to_le_bytes());
        data.extend_from_slice(&1_000_000u64.to_le_bytes()); // end_at
        data.extend_from_slice(&1u64.to_le_bytes()); // interval
        let ix = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(sender.pubkey(), true),
                AccountMeta::new(sender_ata, false),
                AccountMeta::new(escrow.pubkey(), true),
                AccountMeta::new(escrow_token.pubkey(), false),
                AccountMeta::new_readonly(mint.pubkey(), false),
                AccountMeta::new_readonly(token_program_id(), false),
                AccountMeta::new_readonly(system_id(), false),
            ],
        );
        // The escrow keypair co-signs its account creation.
        // NOTE: currently fails inside the .so at its transfer_checked
        // CPI (classic vs wincode Token-2022 encoding; see module doc).
        // The mint/ATA/account construction above this line is verified.
        if let Err(e) = chain.send_signed(&[&sender, &escrow], &[ix]) {
            println!("init_vesting CPI blocked by era mismatch: {e}");
        }

        Ok(Self { program_id, hacker, mint: mint.pubkey(), hacker_ata })
    }

    /// Evaluation: `reveal_secret` succeeds only with the correct secret
    /// (and the expected token state). Failing tx = unsolved.
    pub fn check(&self, chain: &Arc<dyn SolChain>) -> bool {
        // Well-formed call with a wrong secret: fails semantically until
        // the real secret (player-supplied) is used.
        let secret = "unsolved";
        let mut data = discriminator("reveal_secret").to_vec();
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
