//! Ethernaut 11 — Elevator.
//!
//! Goal: Make `Elevator` report `top == true`. Its source also defines the
//! `Building` interface.
//!
//
// Upstream hint:
//
// This elevator won't let you reach the top of your building. Right?
//
// ##### Things that might help:
// * Sometimes solidity is not good at keeping promises.
// * This `Elevator` expects to be used from a `Building`.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Elevator.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl11_elevator::*;

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
