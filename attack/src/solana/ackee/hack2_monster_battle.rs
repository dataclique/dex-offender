//! Ackee Bootcamp 2 — Monster Battle.
//!
//! Goal: Get `reveal_secret` to succeed for BOTH explorers — each needs
//! enough experience and monsters defeated, and the two secrets (which
//! you obtained in level 1) must match. The explorers start
//! uninitialized; initializing them is part of the exploit.
//!
//! Upstream starter (`tests/exploit.ts`): two fresh keypairs (user1,
//! user2) with their `EXPLORER` PDAs pre-derived.
//!
//! Sources (readable source fragment upstream):
//! https://github.com/Ackee-Blockchain/Solana-Auditors-Bootcamp/tree/bc0d25cce79b482599a2dde37c3fb112e17e9d0d/Capture-the-Flag/level-2/src-fragment
//! https://github.com/Ackee-Blockchain/Solana-Auditors-Bootcamp/blob/bc0d25cce79b482599a2dde37c3fb112e17e9d0d/Capture-the-Flag/level-2/target/idl/level_2.json
//! https://github.com/Ackee-Blockchain/Solana-Auditors-Bootcamp/blob/bc0d25cce79b482599a2dde37c3fb112e17e9d0d/Capture-the-Flag/level-2/tests/exploit.ts
//! https://github.com/Ackee-Blockchain/Solana-Auditors-Bootcamp/tree/bc0d25cce79b482599a2dde37c3fb112e17e9d0d/Capture-the-Flag/HINTS/level-2
//! (HINTS are the upstream's progressive spoilers — open only if you
//! want them.)

pub struct Exploit;

pub async fn attack(
    target: &ctf::solana::bootcamp::level_2::Target,
    offender: &ctf::solana::SolActor,
) -> eyre::Result<()> {
    // Starter context from the upstream exploit harness: two fresh
    // keypairs and their explorer PDAs.
    // let user1 = solana_sdk::signature::Keypair::new();
    // let user2 = solana_sdk::signature::Keypair::new();
    // let explorer1 = solana_sdk::pubkey::Pubkey::find_program_address(
    //     &[b"EXPLORER", user1.pubkey().as_ref()],
    //     &target.program_id,
    // )
    // .0;

    // TODO: exploit the level
    let _ = (target, offender);
    Ok(())
}
