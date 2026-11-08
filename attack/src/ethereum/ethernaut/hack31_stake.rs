//! Ethernaut 31 — Stake.
//!
//! Goal: Leave `Stake` with nonzero Ether and `totalStaked` greater than that
//! balance. Your offender account must be recorded in `Stakers` but have zero
//! `UserStake`.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Stake.sol
//!
//! DummyWETH creation and the complete success predicate:
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/StakeFactory.sol

use async_trait::async_trait;
use ctf::ethereum::ethernaut::lvl31_stake::*;

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
