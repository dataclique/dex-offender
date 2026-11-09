//! Ackee Bootcamp 2 — Monster Battle.
//!
//! Goal: Battle monsters, heal allies, and make `reveal_secret` succeed
//! for both explorers. The explorers (and their secrets from level 1)
//! are yours to initialize.
//!
//! Sources:
//! https://github.com/Ackee-Blockchain/Solana-Auditors-Bootcamp/tree/bc0d25cce79b482599a2dde37c3fb112e17e9d0d/Capture-the-Flag/level-2

pub struct Exploit;

pub async fn attack(
    target: &ctf::solana::bootcamp::level_2::Target,
    roles: &ctf::solana::SolRoles,
) -> eyre::Result<()> {
    // TODO: exploit the level
    let _ = (target, roles);
    Ok(())
}
