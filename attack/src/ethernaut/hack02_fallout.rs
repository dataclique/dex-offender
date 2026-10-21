//! Ethernaut level 02 — Fallout.
//!
//! Claim ownership of the contract.
//!
//! See the Fallback example (hack01) for how solutions work here.

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethernaut::lvl02_fallout::Target;

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
