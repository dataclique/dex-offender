//! Ackee 5 — Guardian Escrow.
//!
//! Goal: Obtain the secret from the guardian-protected escrow the
//! setup initializes (classic SPL Token; the verification program is
//! deployed at its canonical address alongside the level program).
//!
//! Sources:
//! https://github.com/Ackee-Blockchain/Solana-Auditors-Bootcamp/tree/bc0d25cce79b482599a2dde37c3fb112e17e9d0d/Capture-the-Flag/level-5

pub struct Exploit;

pub async fn attack(
    target: &ctf::solana::ackee::level_5::Target,
    offender: &ctf::solana::SolActor,
) -> eyre::Result<()> {
    // Persistent setup context: the guardian escrow is initialized on
    // chain (`target.escrow`, `target.escrow_token`, `target.mint`)
    // with the offender as the recipient. When you have extracted the
    // secret, save it as plain text in .solana/answers/ackee-level-5.txt
    // — the runner submits it to the program.

    // TODO: exploit the level
    let _ = (target, offender);
    Ok(())
}
