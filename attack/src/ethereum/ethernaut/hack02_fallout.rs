//! Ethernaut 02 — Fallout.
//!
//! Goal: Become the owner of `Fallout`, the contract that records users' Ether
//! allocations. The owner must be your offender account.
//!
//! Source: [Fallout.sol][source]. [Rust harness][harness].
//! Test: `cargo test -p attack fallout -- --ignored --nocapture`
//!
//! [source]:
//! ../../../../lib/ethernaut/contracts/src/levels/Fallout.sol
//! [harness]: ctf::ethereum::ethernaut::lvl02_fallout

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl02_fallout::Target;

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
