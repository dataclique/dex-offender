//! Solution scaffold for Ethernaut level 29 — switch.
//!
//! Call via off-chain lookUp with flipped selector.

use async_trait::async_trait;
use ctf::Actor;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = ctf::ethernaut::lvl29_switch::Target;

    async fn attack(self, target: &Self::Target, offender: &Actor) -> eyre::Result<()> {
        // TODO: exploit the level
        let _ = (target, offender);
        Ok(())
    }
}
