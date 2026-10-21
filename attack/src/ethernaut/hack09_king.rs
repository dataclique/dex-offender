//! Solution scaffold for Ethernaut level 09 — king.
//!
//! Prevent prize transfer to stay king forever.

use async_trait::async_trait;
use ctf::Actor;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = ctf::ethernaut::lvl09_king::Target;

    async fn attack(self, target: &Self::Target, offender: &Actor) -> eyre::Result<()> {
        // TODO: exploit the level
        let _ = (target, offender);
        Ok(())
    }
}
