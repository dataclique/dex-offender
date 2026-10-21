//! Solution scaffold for Ethernaut level 14 — gatekeeper_two.
//!
//! Pass 3 gates with extcodesize + truncation.

use async_trait::async_trait;
use ctf::Actor;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = ctf::ethereum::ethernaut::lvl14_gatekeeper_two::Target;

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
