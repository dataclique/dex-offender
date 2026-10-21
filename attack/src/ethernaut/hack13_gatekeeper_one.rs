//! Solution scaffold for Ethernaut level 13 — gatekeeper_one.
//!
//! Pass 3 gates with gas tuning + type conversion.

use async_trait::async_trait;
use ctf::Actor;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = ctf::ethernaut::lvl13_gatekeeper_one::Target;

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
