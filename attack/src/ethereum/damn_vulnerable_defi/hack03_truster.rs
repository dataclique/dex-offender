//! Solution scaffold for DVD — Truster.
//!
//! Use the flash loan callback to steal all tokens from the pool.

pub struct Exploit;

pub async fn attack(
    target: &ctf::ethereum::damn_vulnerable_defi::truster::Target,
    offender: &ctf::Actor,
) -> eyre::Result<()> {
    let _ = (target, offender);
    // TODO: exploit the level
    Ok(())
}
