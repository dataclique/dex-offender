//! Solution scaffold for Damn Vulnerable DeFi — Unstoppable.
//!
//! Make the vault stop offering flash loans.

pub struct Exploit;

pub async fn attack(
    target: &ctf::ethereum::damn_vulnerable_defi::unstoppable::Target,
    offender: &ctf::Actor,
) -> eyre::Result<()> {
    // TODO: exploit the level
    let _ = (target, offender);
    Ok(())
}
