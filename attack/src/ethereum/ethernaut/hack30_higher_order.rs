//! Ethernaut 30 — Higher Order.
//!
//! Goal: Make your offender account the `commander` of `HigherOrder`.
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
