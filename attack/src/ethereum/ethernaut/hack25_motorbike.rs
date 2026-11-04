//! Ethernaut 25 — Motorbike.
//!
//! Goal: Leave the original `Engine` implementation behind the `Motorbike`
//! proxy without contract code.
//!
//
// Upstream hint:
//
// Ethernaut's motorbike has a brand new upgradeable engine design.
//
// Would you be able to `selfdestruct` its engine and make the motorbike
// unusable ?
//
// Things that might help:
//
// - [EIP-1967](https://eips.ethereum.org/EIPS/eip-1967)
// -
// [UUPS](https://forum.openzeppelin.com/t/uups-proxies-tutorial-solidity-javascript/7786)
// upgradeable pattern
// -
// [Initializable](https://github.com/OpenZeppelin/openzeppelin-upgrades/blob/master/packages/core/contracts/Initializable.sol)
// contract
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Motorbike.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl25_motorbike::*;

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
