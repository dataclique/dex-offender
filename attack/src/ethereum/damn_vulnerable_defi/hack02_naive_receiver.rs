//! Solution scaffold for DVD — Naive Receiver.
//!
//! Drain all WETH from the pool and receiver.

use ctf::Actor;
use ctf::ethereum::damn_vulnerable_defi::naive_receiver::Target;

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
