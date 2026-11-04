//! Ethernaut 06 — Delegation.
//!
//! Goal: Take ownership of the `Delegation` instance, which uses a companion
//! `Delegate` contract. The checked owner belongs to `Delegation`, not
//! `Delegate`.
//!
//
// Upstream hint:
//
// The goal of this level is for you to claim ownership of the instance you
// are given.
//
// Things that might help
// * Look into Solidity's documentation on the `delegatecall` low level
// function, how it works, how it can be used to delegate operations to
// on-chain libraries, and what implications it has on execution scope.
// * Fallback methods
// * Method ids
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Delegation.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl06_delegate::*;

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
