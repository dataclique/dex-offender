//! Solution scaffold for Ethernaut level 10 — reentrancy.
//!
//! Drain via reentrant withdraw call.

use async_trait::async_trait;
use ctf::Actor;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = ctf::ethernaut::lvl10_reentrancy::Target;

    async fn attack(self, target: &Self::Target, offender: &Actor) -> eyre::Result<()> {
        // TODO: exploit the level
        let _ = (target, offender);
        Ok(())
    }
}
