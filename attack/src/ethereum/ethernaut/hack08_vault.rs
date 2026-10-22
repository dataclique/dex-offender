//! Solution scaffold for Ethernaut level 08 — vault.
//!
//! Read the private password from storage.

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl08_vault::Target;

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
