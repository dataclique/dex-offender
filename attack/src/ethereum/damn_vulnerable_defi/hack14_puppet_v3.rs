//! Solution scaffold for DVD — PuppetV3.
//!
//! Drain the pool by manipulating the UniswapV3 price.

use ctf::Actor;
use ctf::ethereum::damn_vulnerable_defi::puppet_v3::Target;

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
