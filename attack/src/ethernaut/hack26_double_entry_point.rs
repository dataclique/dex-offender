//! Solution scaffold for Ethernaut level 26 — double_entry_point.
//!
//! Deploy Forta bot to raise alerts.

use async_trait::async_trait;
use ctf::Actor;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = ctf::ethernaut::lvl26_double_entry_point::Target;

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
