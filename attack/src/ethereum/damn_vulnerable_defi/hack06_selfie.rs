//! Solution scaffold for DVD — Selfie.
//!
//! Use a flash loan to gain voting power and drain the pool.

use ctf::Actor;
use ctf::ethereum::damn_vulnerable_defi::selfie::Target;

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
