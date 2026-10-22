//! Solution scaffold for DVD — The Rewarder.
//!
//! Claim all rewards from the distributor.

pub struct Exploit;

pub async fn attack(
    target: &ctf::ethereum::damn_vulnerable_defi::the_rewarder::Target,
    offender: &ctf::Actor,
) -> eyre::Result<()> {
    let _ = (target, offender);
    // TODO: exploit the level
    Ok(())
}
