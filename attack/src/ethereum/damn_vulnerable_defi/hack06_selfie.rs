//! Solution scaffold for DVD — Selfie.
//!
//! Use a flash loan to gain voting power and drain the pool.

pub struct Exploit;

pub async fn attack(
    target: &ctf::ethereum::damn_vulnerable_defi::selfie::Target,
    offender: &ctf::Actor,
) -> eyre::Result<()> {
    let _ = (target, offender);
    // TODO: exploit the level
    Ok(())
}
