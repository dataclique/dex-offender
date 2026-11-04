//! Ethernaut 20 — Denial.
//!
//! Goal: Prevent `Denial.withdraw()` from succeeding with a gas limit of
//! 1,000,000 while the contract still holds more than 100 wei.
//!
//
// Upstream hint:
//
// This is a simple wallet that drips funds over time. You can withdraw the
// funds
// slowly by becoming a withdrawing partner.
//
// If you can deny the owner from withdrawing funds when they call
// `withdraw()`
// (whilst the contract still has funds, and the transaction is of 1M gas or
// less) you will win this level.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Denial.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl20_denial::*;

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
