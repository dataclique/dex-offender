//! Ethernaut 03 — Coin Flip.
//!
//! Goal: Win at least ten consecutive guesses in the `CoinFlip` game without
//! resetting the winning streak.
//!
//! Source: [CoinFlip.sol][source]. [Rust harness][harness].
//! Test: `cargo test -p attack coin_flip -- --ignored --nocapture`
//!
//! [source]:
//! ../../../../lib/ethernaut/contracts/src/levels/CoinFlip.sol
//! [harness]: ctf::ethereum::ethernaut::lvl03_coin_flip

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
