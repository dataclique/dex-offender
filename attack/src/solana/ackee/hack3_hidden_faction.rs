//! Ackee Bootcamp 3 — Hidden Faction.
//!
//! Goal: Join the faction, obtain its token, and reveal the faction
//! secret.
//!
//! Sources:
//! https://github.com/Ackee-Blockchain/Solana-Auditors-Bootcamp/tree/bc0d25cce79b482599a2dde37c3fb112e17e9d0d/Capture-the-Flag/level-3

pub struct Exploit;

pub async fn attack(
    target: &ctf::solana::bootcamp::level_3::Target,
    roles: &ctf::solana::SolRoles,
) -> eyre::Result<()> {
    // TODO: exploit the level
    let _ = (target, roles);
    Ok(())
}
