//! Ethernaut 31 — Stake.
//!
//! Goal: Leave `Stake` with nonzero Ether and `totalStaked` greater than that
//! balance. Your offender account must be recorded in `Stakers` but have zero
//! `UserStake`.
//!
//
// Upstream hint:
//
// Stake is safe for staking native ETH and ERC20 WETH, considering the same
// 1:1 value of the tokens. Can you drain the contract?
//
// To complete this level, the contract state must meet the following
// conditions:
//
// - The `Stake` contract's ETH balance has to be greater than 0.
// - `totalStaked` must be greater than the `Stake` contract's ETH balance.
// - You must be a staker.
// - Your staked balance must be 0.
//
// Things that might be useful:
//
// - [ERC-20](https://github.com/ethereum/ercs/blob/master/ERCS/erc-20.md)
// specification.
// - [OpenZeppelin
// contracts](https://github.com/OpenZeppelin/openzeppelin-contracts)
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
