//! Ethernaut 21 — Shop.
//!
//! Goal: Buy the item from `Shop` for less than its initial asking price of
//! 100.
//!
//
// Upstream hint:
//
// Сan you get the item from the shop for less than the price asked?
//
// ##### Things that might help:
// * `Shop` expects to be used from a `Buyer`
// * Understanding restrictions of view functions
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Shop.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl21_shop::*;

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
