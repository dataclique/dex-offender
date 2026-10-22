//! Solution scaffold for Ethernaut level 29 — switch.
//!
//! Call via off-chain lookUp with flipped selector.

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl29_switch::Target;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        // TODO: exploit the level
        let _ = (target, offender);
        Ok(())
    }
}
