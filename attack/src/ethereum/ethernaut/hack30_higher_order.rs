//! Ethernaut 30 — Higher Order.
//!
//! Goal: Make your offender account the `commander` of `HigherOrder`.
//!
//
// Upstream hint:
//
// Imagine a world where the rules are meant to be broken, and only the
// cunning and the bold can rise to power. Welcome to the Higher Order, a
// group shrouded in mystery, where a treasure awaits and a commander rules
// supreme.
//
// Your objective is to become the Commander of the Higher Order! Good luck!
//
// ##### Things that might help:
// * Sometimes, `calldata` cannot be trusted.
// * Compilers are constantly evolving into better spaceships.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/HigherOrder.sol

use async_trait::async_trait;
use ctf::ethereum::ethernaut::lvl30_higher_order::*;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        // TODO: exploit the level.
        let _ = (target, offender);
        Ok(())
    }
}
