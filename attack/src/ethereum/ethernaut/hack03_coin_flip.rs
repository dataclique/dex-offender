//! Ethernaut 03 — Coin Flip.
//!
//! Goal: Win at least ten consecutive guesses in the `CoinFlip` game without
//! resetting the winning streak.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/CoinFlip.sol
//! Test: `cargo test -p attack coin_flip -- --ignored --nocapture`

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
