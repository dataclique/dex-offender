//! Solution scaffold for DVD — Naive Receiver.
//!
//! Drain all WETH from the pool and receiver.

pub struct Exploit;

pub async fn attack(
    target: &ctf::ethereum::damn_vulnerable_defi::naive_receiver::Target,
    offender: &ctf::Actor,
) -> eyre::Result<()> {
    let _ = (target, offender);
    // TODO: exploit the level
    Ok(())
}
