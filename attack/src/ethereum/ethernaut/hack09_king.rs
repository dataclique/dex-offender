//! Ethernaut level 09 — King.
//!
//! Goal: Claim the throne in `King` and prevent the original owner from reclaiming it during the level check.
//!
//! Source: [local](../../../../lib/ethernaut/contracts/src/levels/King.sol) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/contracts/src/levels/King.sol).
//! Challenge: [local](../../../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/king.md) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/client/src/gamedata/en/descriptions/levels/king.md).
//! [Factory success check](../../../../lib/ethernaut/contracts/src/levels/KingFactory.sol) · [Rust bindings](ctf::ethereum::ethernaut::lvl09_king).
//!
//! `target` contains the deployed instance address and any level-specific helpers.
//! `offender` is your player account; sign transactions with `offender.address()`.

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl09_king::Target;

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
