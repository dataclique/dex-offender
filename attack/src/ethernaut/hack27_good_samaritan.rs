//! Solution scaffold for Ethernaut level 27 — good_samaritan.
//!
//! Revert on coin transfer to drain wallet.

use async_trait::async_trait;
use ctf::Actor;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = ctf::ethernaut::lvl27_good_samaritan::Target;

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
