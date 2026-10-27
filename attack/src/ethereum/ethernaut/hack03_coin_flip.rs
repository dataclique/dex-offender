//! Ethernaut level 03 — Coin Flip.
//!
//! Goal: Win at least ten consecutive guesses in the `CoinFlip` game without resetting the winning streak.
//!
//! Source: [local](../../../../lib/ethernaut/contracts/src/levels/CoinFlip.sol) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/contracts/src/levels/CoinFlip.sol).
//! Challenge: [local](../../../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/coinflip.md) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/client/src/gamedata/en/descriptions/levels/coinflip.md).
//! [Factory success check](../../../../lib/ethernaut/contracts/src/levels/CoinFlipFactory.sol) · [Rust bindings](ctf::ethereum::ethernaut::lvl03_coin_flip).
//!
//! `target` contains the deployed instance address and any level-specific helpers.
//! `offender` is your player account; sign transactions with `offender.address()`.

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl03_coin_flip::Target;

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
