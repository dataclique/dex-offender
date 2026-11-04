//! Ethernaut 03 — Coin Flip.
//!
//! Goal: Win at least ten consecutive guesses in the `CoinFlip` game without
//! resetting the winning streak.
//!
//
// Upstream hint:
//
// This is a coin flipping game where you need to build up your winning
// streak by guessing the outcome of a coin flip. To complete this level
// you'll need to use your psychic abilities to guess the correct outcome 10
// times in a row.
//
// Things that might help
// * See the ["?"](https://ethernaut.openzeppelin.com/help) page above in the
// top right corner menu, section "Beyond the console"
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/CoinFlip.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl03_coin_flip::*;

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
