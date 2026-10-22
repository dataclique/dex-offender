//! Solution scaffold for DVD — Climber.
//!
//! Drain the ClimberVault by exploiting the timelock's scheduling logic.

use ctf::Actor;
use ctf::ethereum::damn_vulnerable_defi::climber::Target;

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
