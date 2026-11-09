//! Ackee Bootcamp 3 — Hidden Faction.
//!
//! Goal: Make `show_faction_secret` succeed for a member who holds
//! exactly one faction token — join the faction, obtain the token, and
//! reveal the secret.
//!
//! Upstream starter (`tests/exploit.ts`): one fresh member keypair.
//!
//! Sources (readable source fragment upstream):
//! https://github.com/Ackee-Blockchain/Solana-Auditors-Bootcamp/tree/bc0d25cce79b482599a2dde37c3fb112e17e9d0d/Capture-the-Flag/level-3/src-fragment
//! https://github.com/Ackee-Blockchain/Solana-Auditors-Bootcamp/blob/bc0d25cce79b482599a2dde37c3fb112e17e9d0d/Capture-the-Flag/level-3/target/idl/level_3.json
//! https://github.com/Ackee-Blockchain/Solana-Auditors-Bootcamp/blob/bc0d25cce79b482599a2dde37c3fb112e17e9d0d/Capture-the-Flag/level-3/tests/exploit.ts
//! https://github.com/Ackee-Blockchain/Solana-Auditors-Bootcamp/tree/bc0d25cce79b482599a2dde37c3fb112e17e9d0d/Capture-the-Flag/HINTS/level-3
//! (HINTS are the upstream's progressive spoilers — open only if you
//! want them.)

pub struct Exploit;

pub async fn attack(
    target: &ctf::solana::bootcamp::level_3::Target,
    roles: &ctf::solana::SolRoles,
) -> eyre::Result<()> {
    // Starter context from the upstream exploit harness: a fresh member
    // keypair (roles.offender is funded and usable directly).

    // TODO: exploit the level
    let _ = (target, roles);
    Ok(())
}
