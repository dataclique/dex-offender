//! Ethernaut level 02 — Fallout.
//!
//! Goal: Become the owner of `Fallout`, the contract that records users' Ether allocations. The owner must be your offender account.
//!
//! Source: [local](../../../../lib/ethernaut/contracts/src/levels/Fallout.sol) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/contracts/src/levels/Fallout.sol).
//! Challenge: [local](../../../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/fallout.md) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/client/src/gamedata/en/descriptions/levels/fallout.md).
//! [Factory success check](../../../../lib/ethernaut/contracts/src/levels/FalloutFactory.sol) · [Rust bindings](ctf::ethereum::ethernaut::lvl02_fallout).
//!
//! `target` contains the deployed instance address and any level-specific helpers.
//! `offender` is your player account; sign transactions with `offender.address()`.

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl02_fallout::Target;

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
