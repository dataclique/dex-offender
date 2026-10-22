//! Solution scaffold for DVD — Unstoppable.
//!
//! Make the vault stop offering flash loans.

use ctf::Actor;
use ctf::ethereum::damn_vulnerable_defi::unstoppable::Target;

pub struct Exploit;

#[async_trait::async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        let _ = (target, offender);
        // TODO: exploit the level
        Ok(())
    }
}
