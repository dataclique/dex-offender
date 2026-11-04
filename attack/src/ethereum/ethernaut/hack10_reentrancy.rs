//! Ethernaut 10 — Re-entrancy.
//!
//! Goal: Empty the Ether held by `Reentrance`, a contract that records
//! donations and lets users withdraw their balances.
//!
//
// Upstream hint:
//
// The goal of this level is for you to steal all the funds from the
// contract.
//
// Things that might help:
// * Untrusted contracts can execute code where you least expect it.
// * Fallback methods
// * Throw/revert bubbling
// * Sometimes the best way to attack a contract is with another contract.
// * See the ["?"](https://ethernaut.openzeppelin.com/help) page above,
// section "Beyond the console"
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Reentrance.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl10_reentrancy::*;

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
