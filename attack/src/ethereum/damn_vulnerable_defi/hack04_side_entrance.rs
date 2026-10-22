//! Solution scaffold for DVD — Side Entrance.
//!
//! Use the pool's deposit/withdraw to drain all ETH.

pub struct Exploit;

pub async fn attack(
    target: &ctf::ethereum::damn_vulnerable_defi::side_entrance::Target,
    offender: &ctf::Actor,
) -> eyre::Result<()> {
    let _ = (target, offender);
    // TODO: exploit the level
    Ok(())
}
