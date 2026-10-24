//! Paradigm CTF 2022 — hana (Solana), challenge one: fake bitcoin.
//!
//! Mirrors the upstream `server/src/challenge.rs` setup: deploy the
//! prebuilt challenge program at its canonical ID, run the raw `setup`
//! and `deposit` instructions (authority/satoshi signers), and hand the
//! player a bitcoin + voucher account pair. Win = the deposit account's
//! token balance is zero (all deposited funds stolen).
//!
//! Threat-model note: the prebuilt `.so` executes only inside the
//! in-process LiteSVM sandbox (ephemeral, offline, dev keypairs). The
//! single deserialization boundary is `check()` over raw account bytes;
//! malformed data reads as unsolved, never panics (pinned by tests).

use crate::solana::SolChain;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use std::sync::Arc;

/// Canonical challenge-one program ID (from the upstream server source).
pub const PROGRAM_ID: &str = "9NK3PnfnqMihgTbNuqjWCL6t8F4kPLqSz1wuj9wYxsoh";

/// spl-associated-token-account program (fixed, well-known address).
const ATA_PROGRAM: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
const TOKEN_PROGRAM: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

/// Raw anchor discriminators captured from the upstream server.
const SETUP_DATA: &[u8] = &[0xd0, 0x99, 0x4d, 0x49, 0xf2, 0xb9, 0x96, 0x4e];
const DEPOSIT_DATA_PREFIX: &[u8] =
    &[0xf2, 0x23, 0xc6, 0x89, 0x52, 0xe1, 0xf2, 0xb6];
const BITCOIN_DECIMALS: u8 = 6;
/// 1.0 bitcoin with 6 decimals (0x0f4240), little-endian u64.
const ONE_BITCOIN: u64 = 1_000_000;

fn pda(program: &Pubkey, seeds: &[&[u8]]) -> Pubkey {
    Pubkey::find_program_address(seeds, program).0
}

/// Associated token account address for (owner, mint) under spl-token.
fn associated_token_address(owner: &Pubkey, mint: &Pubkey) -> Pubkey {
    let token_program: Pubkey = TOKEN_PROGRAM.parse().unwrap();
    let ata_program: Pubkey = ATA_PROGRAM.parse().unwrap();
    pda(&ata_program, &[owner.as_ref(), token_program.as_ref(), mint.as_ref()])
}

#[derive(Clone)]
pub struct Target {
    pub program_id: Pubkey,
    pub deposit_account: Pubkey,
}

impl Target {
    pub fn set_up(
        chain: &Arc<dyn SolChain>,
        roles: &crate::solana::SolRoles,
    ) -> eyre::Result<Self> {
        let program_id = PROGRAM_ID.parse::<Pubkey>()?;
        let so = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../lib/paradigm-ctf-2022/hana/elf/challenge1.so"
        );
        let elf = std::fs::read(so)?;
        // solana-1.9-era program: deploy under BPFLoader2
        // (`solana_sdk::bpf_loader::ID`), not the upgradeable loader.
        chain.deploy_program_with_loader(
            program_id,
            &elf,
            solana_sdk::bpf_loader::ID,
        )?;

        let player = roles.offender.pubkey();
        let authority = Keypair::new();
        let satoshi = Keypair::new();
        let bitcoin = Keypair::new();
        let token_program: Pubkey = TOKEN_PROGRAM.parse()?;

        let state = pda(&program_id, &[player.as_ref(), b"STATE"]);
        let deposit_account = pda(&program_id, &[player.as_ref(), b"TOKEN"]);
        let voucher_mint = pda(&program_id, &[player.as_ref(), b"VOUCHER"]);

        // Fund everyone (rent for mints/ATAs + fees).
        chain.airdrop(&authority.pubkey(), 10_000_000_000)?;
        chain.airdrop(&satoshi.pubkey(), 10_000_000_000)?;
        chain.airdrop(&player, 10_000_000_000)?;

        // Create + initialize the bitcoin mint (authority = satoshi).
        create_mint(chain, &satoshi, &bitcoin, BITCOIN_DECIMALS)?;

        // Program `setup` (authority signs; PDAs created by the program).
        let setup_ixn = Instruction::new_with_bytes(
            program_id,
            SETUP_DATA,
            vec![
                AccountMeta::new_readonly(player, false),
                AccountMeta::new(authority.pubkey(), true),
                AccountMeta::new(state, false),
                AccountMeta::new(deposit_account, false),
                AccountMeta::new_readonly(bitcoin.pubkey(), false),
                AccountMeta::new(voucher_mint, false),
                AccountMeta::new_readonly(rent_id(), false),
                AccountMeta::new_readonly(system_id(), false),
                AccountMeta::new_readonly(token_program, false),
            ],
        );
        chain.send(&authority, &[setup_ixn])?;

        // Satoshi's token accounts + one bitcoin.
        let satoshi_bitcoin =
            ensure_ata(chain, &satoshi.pubkey(), &satoshi, &bitcoin.pubkey())?;
        let satoshi_voucher =
            ensure_ata(chain, &satoshi.pubkey(), &satoshi, &voucher_mint)?;
        mint_to(
            chain,
            &satoshi,
            &bitcoin.pubkey(),
            &satoshi_bitcoin,
            ONE_BITCOIN,
        )?;

        // Program `deposit` (satoshi signs).
        let mut deposit_data = DEPOSIT_DATA_PREFIX.to_vec();
        deposit_data.extend_from_slice(&ONE_BITCOIN.to_le_bytes());
        let deposit_ixn = Instruction::new_with_bytes(
            program_id,
            &deposit_data,
            vec![
                AccountMeta::new_readonly(player, false),
                AccountMeta::new_readonly(satoshi.pubkey(), true),
                AccountMeta::new_readonly(state, false),
                AccountMeta::new(deposit_account, false),
                AccountMeta::new(voucher_mint, false),
                AccountMeta::new(satoshi_bitcoin, false),
                AccountMeta::new(satoshi_voucher, false),
                AccountMeta::new_readonly(token_program, false),
            ],
        );
        chain.send(&satoshi, &[deposit_ixn])?;

        // Player's token accounts + one bitcoin to play with.
        let player_bitcoin =
            ensure_ata(chain, &player, &authority, &bitcoin.pubkey())?;
        ensure_ata(chain, &player, &authority, &voucher_mint)?;
        mint_to(
            chain,
            &satoshi,
            &bitcoin.pubkey(),
            &player_bitcoin,
            ONE_BITCOIN,
        )?;

        Ok(Self { program_id, deposit_account })
    }

    /// Win condition (upstream `check_win`): the deposit account holds
    /// no tokens. Raw account bytes cross the only deserialization
    /// boundary — malformed data reads as unsolved, never panics.
    pub fn check(&self, chain: &Arc<dyn SolChain>) -> bool {
        let Some(data) = chain.get_account(&self.deposit_account) else {
            return false;
        };
        // spl-token Account packs `amount` at offset 64.
        data.len() >= 72 && data[64..72] == [0u8; 8]
    }
}

/// Classic system program (1111…1) address.
fn system_id() -> Pubkey {
    // Canonical system program: base58 "1111…1" = all-zero bytes.
    Pubkey::new_from_array([0u8; 32])
}

/// Rent sysvar (SysvarRent111111111111111111111111111111111).
fn rent_id() -> Pubkey {
    "SysvarRent111111111111111111111111111111111".parse().unwrap()
}

fn token_program_id() -> Pubkey {
    TOKEN_PROGRAM.parse().unwrap()
}

/// Create + initialize an spl-token mint; the mint keypair co-signs its
/// own account creation. All three instructions are hand-encoded with
/// their canonical discriminators (spl-token 8.0.0's crate types are
/// incompatible with our solana-sdk 4 instruction types).
fn create_mint(
    chain: &Arc<dyn SolChain>,
    authority: &Keypair,
    mint: &Keypair,
    decimals: u8,
) -> eyre::Result<()> {
    use solana_sdk::{message::Message, transaction::Transaction};

    let create_ixn = solana_system_interface::instruction::create_account(
        &authority.pubkey(),
        &mint.pubkey(),
        1_000_000_000, // comfortably above rent
        82,
        &token_program_id(),
    );

    // spl-token InitializeMint — encoded by the spl-token 8.0.0 crate
    // (authoritative layout), types bridged across the instruction
    // version gap via bytes.
    let bridge = |pk: &Pubkey| {
        spl_token::solana_program::pubkey::Pubkey::new_from_array(pk.to_bytes())
    };
    let init_ixn_theirs = spl_token::instruction::initialize_mint(
        &bridge(&token_program_id()),
        &bridge(&mint.pubkey()),
        &bridge(&authority.pubkey()),
        None,
        decimals,
    )?;
    let init_data = init_ixn_theirs.data;
    let init_ixn = Instruction::new_with_bytes(
        token_program_id(),
        &init_data,
        vec![
            AccountMeta::new(mint.pubkey(), false),
            AccountMeta::new_readonly(rent_id(), false),
        ],
    );

    let payer = authority.pubkey();
    let msg = Message::new(&[create_ixn, init_ixn], Some(&payer));
    let tx =
        Transaction::new(&[authority, mint], msg, chain.latest_blockhash());
    chain.send_transaction(tx).map(|_| ())
}

/// Create (idempotent) the associated token account; returns its address.
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
    // spl-token MintTo — encoded by the spl-token 8.0.0 crate, bridged
    // across the instruction version gap via bytes.
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
    let data = ixn_theirs.data;
    let ixn = Instruction::new_with_bytes(
        token_program_id(),
        &data,
        vec![
            AccountMeta::new(*mint, false),
            AccountMeta::new(*target, false),
            AccountMeta::new_readonly(authority.pubkey(), true),
        ],
    );
    chain.send(authority, &[ixn]).map(|_| ())
}
