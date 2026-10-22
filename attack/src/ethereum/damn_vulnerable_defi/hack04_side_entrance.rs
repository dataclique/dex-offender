//! Solution scaffold for DVD — Side Entrance.
//!
//! Use the pool's deposit/withdraw to drain all ETH.

use ctf::Actor;
use ctf::ethereum::damn_vulnerable_defi::side_entrance::Target;

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
