//! Ethernaut 07 — Force.
//!
//! Goal: Give the otherwise empty `Force` contract a positive Ether balance.
//!
//
// Upstream hint:
//
// Some contracts will simply not take your money `¯\_(ツ)_/¯`
//
// The goal of this level is to make the balance of the contract greater than
// zero.
//
// Things that might help:
// * Fallback methods
// * Sometimes the best way to attack a contract is with another contract.
// * See the ["?"](https://ethernaut.openzeppelin.com/help) page above,
// section "Beyond the console"
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Force.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl07_force::*;

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
