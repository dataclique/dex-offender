//! Ackee Bootcamp 4 — Vesting.
//!
//! Goal: Initialize the vesting escrow, then withdraw early so
//! `reveal_secret(secret)` succeeds for the holder.
//! Harness status: partially playable — the pinned program's
//! Token-2022 CPI encoding is rejected by the runtime's bundled token
//! program (see ctf/src/solana/bootcamp/level_4.rs for the exact
//! blocker); everything up to that CPI is verified working.
//!
//! Sources:
//! https://github.com/Ackee-Blockchain/Solana-Auditors-Bootcamp/tree/bc0d25cce79b482599a2dde37c3fb112e17e9d0d/Capture-the-Flag/level-4

pub struct Exploit;

pub async fn attack(
    target: &ctf::solana::bootcamp::level_4::Target,
    roles: &ctf::solana::SolRoles,
) -> eyre::Result<()> {
    // TODO: exploit the level
    let _ = (target, roles);
    Ok(())
}
