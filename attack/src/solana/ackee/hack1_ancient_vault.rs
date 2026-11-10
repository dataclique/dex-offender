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
//! that is `offender` here — and the vault PDA derivation.
//!
//! Sources:
//! https://github.com/Ackee-Blockchain/Solana-Auditors-Bootcamp/blob/bc0d25cce79b482599a2dde37c3fb112e17e9d0d/Capture-the-Flag/level-1/target/idl/level_1.json
//! https://github.com/Ackee-Blockchain/Solana-Auditors-Bootcamp/blob/bc0d25cce79b482599a2dde37c3fb112e17e9d0d/Capture-the-Flag/level-1/tests/exploit.ts
//! https://github.com/Ackee-Blockchain/Solana-Auditors-Bootcamp/tree/bc0d25cce79b482599a2dde37c3fb112e17e9d0d/Capture-the-Flag/HINTS/level-1
//! (HINTS are the upstream's progressive spoilers — open only if you
//! want them.)

pub struct Exploit;

pub async fn attack(
    target: &ctf::solana::bootcamp::level_1::Target,
    offender: &ctf::solana::SolActor,
) -> eyre::Result<()> {
    // Starter context from the upstream exploit harness:
    // offender is the funded fresh keypair; the vault PDA is
    // let vault = target.vault_pda();

    // TODO: exploit the level
    let _ = (target, offender);
    Ok(())
}
