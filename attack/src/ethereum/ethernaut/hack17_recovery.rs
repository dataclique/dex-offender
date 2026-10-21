//! Solution scaffold for Ethernaut level 17 — recovery.
//!
//! Compute CREATE address from nonce+deployer.

use async_trait::async_trait;
use ctf::Actor;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = ctf::ethereum::ethernaut::lvl17_recovery::Target;

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
