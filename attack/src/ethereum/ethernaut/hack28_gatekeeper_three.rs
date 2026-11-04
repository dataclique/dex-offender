//! Ethernaut 28 — Gatekeeper Three.
//!
//! Goal: Pass the gates of `GatekeeperThree`, which uses a `SimpleTrick`
//! helper, and register your offender account as its `entrant`.
//!
//
// Upstream hint:
//
// Cope with gates and become an entrant.
//
// ##### Things that might help:
// * Recall return values of low-level functions.
// * Be attentive with semantic.
// * Refresh how storage works in Ethereum.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/GatekeeperThree.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl28_gatekeeper_three::*;

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
