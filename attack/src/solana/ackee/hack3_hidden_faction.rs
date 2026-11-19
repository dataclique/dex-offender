//! Ackee 3 — Hidden Faction.
//!
//! Goal: Make `show_faction_secret` succeed for a member who holds
//! exactly one faction token — join the faction, obtain the token, and
//! reveal the secret. The offender is the joining member; the faction
//! authority that co-signs joins is exposed by the target.
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
    target: &ctf::solana::ackee::level_3::Target,
    offender: &ctf::solana::SolActor,
) -> eyre::Result<()> {
    // Persistent setup context: the faction and its mint are
    // initialized on chain (`target.faction`, `target.mint`);
    // `target.faction_authority` co-signs `obtain_faction_token`, and
    // the offender is the joining member. When you have extracted the
    // secret, save it as plain text in
    // .solana/answers/ackee-level-3.txt — the runner submits it to the
    // program.

    // TODO: exploit the level
    let _ = (target, offender);
    Ok(())
}
