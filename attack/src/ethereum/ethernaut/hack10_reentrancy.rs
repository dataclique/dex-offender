//! Ethernaut level 10 — Re-entrancy.
//!
//! Goal: Empty the Ether held by `Reentrance`, a contract that records donations and lets users withdraw their balances.
//!
//! Source: [local](../../../../lib/ethernaut/contracts/src/levels/Reentrance.sol) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/contracts/src/levels/Reentrance.sol).
//! Challenge: [local](../../../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/reentrancy.md) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/client/src/gamedata/en/descriptions/levels/reentrancy.md).
//! [Factory success check](../../../../lib/ethernaut/contracts/src/levels/ReentranceFactory.sol) · [Rust bindings](ctf::ethereum::ethernaut::lvl10_reentrancy).
//!
//! `target` contains the deployed instance address and any level-specific helpers.
//! `offender` is your player account; sign transactions with `offender.address()`.

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl10_reentrancy::Target;

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
