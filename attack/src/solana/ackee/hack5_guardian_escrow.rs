//! Ackee Bootcamp 5 — Guardian Escrow.
//!
//! Goal: Initialize the guardian-protected escrow, then obtain the
//! secret.
//!
//! Sources:
//! https://github.com/Ackee-Blockchain/Solana-Auditors-Bootcamp/tree/bc0d25cce79b482599a2dde37c3fb112e17e9d0d/Capture-the-Flag/level-5

pub struct Exploit;

pub async fn attack(
    target: &ctf::solana::bootcamp::level_5::Target,
    offender: &ctf::solana::SolActor,
) -> eyre::Result<()> {
    // TODO: exploit the level
    let _ = (target, offender);
    Ok(())
}
