//! Ackee 4 — Vesting.
//!
//! Goal: Withdraw early from the initialized vesting escrow so
//! `reveal_secret(secret)` succeeds for the holder. Setup initializes
//! the escrow on the persistent chain; if the runtime's Token-2022
//! rejects the pinned program's CPI encoding, deployment reports the
//! exact failure and the runner marks this level blocked.
//!
//! Sources:
//! https://github.com/Ackee-Blockchain/Solana-Auditors-Bootcamp/tree/bc0d25cce79b482599a2dde37c3fb112e17e9d0d/Capture-the-Flag/level-4

pub struct Exploit;

pub async fn attack(
    target: &ctf::solana::ackee::level_4::Target,
    offender: &ctf::solana::SolActor,
) -> eyre::Result<()> {
    // Persistent setup context: the escrow is initialized on chain
    // (`target.escrow`, `target.escrow_token`, `target.mint`) with the
    // offender as the vesting recipient. When you have extracted the
    // secret, save it as plain text in .solana/answers/ackee-level-4.txt
    // — the runner submits it to the program.

    // TODO: exploit the level
    let _ = (target, offender);
    Ok(())
}
