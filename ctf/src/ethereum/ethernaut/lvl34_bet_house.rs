//! Ethernaut 34 — Bet House.
//!
//! Goal: Become a bettor in `BetHouse`, which uses a `Pool` and two `PoolToken`
//! contracts. Your offender account receives five deposit tokens at setup;
//! `isBettor(offender)` must become true.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/BetHouse.sol

use crate::{Level, Roles};
use alloy::{primitives::Address, primitives::U256, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    BetHouse,
    "abi/ethernaut/BetHouse.json"
);

sol!(
    #[sol(rpc)]
    Pool,
    "abi/ethernaut/Pool.json"
);

sol!(
    #[sol(rpc)]
    PoolToken,
    "abi/ethernaut/PoolToken.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Bet House"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level34.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let offender = &roles.offender;
        let provider = roles.provider();

        // Mirror BetHouseFactory.createInstance.
        let wrapped = PoolToken::deploy(
            provider,
            "PoolWrappedToken".into(),
            "PWT".into(),
        )
        .await?;
        let deposit = PoolToken::deploy(
            provider,
            "PoolDepositToken".into(),
            "PDT".into(),
        )
        .await?;
        let pool =
            Pool::deploy(provider, *wrapped.address(), *deposit.address())
                .await?;
        let contract = BetHouse::deploy(provider, *pool.address()).await?;

        deposit
            .mint(offender.address(), U256::from(5))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        wrapped
            .transferOwnership(*pool.address())
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        deposit
            .transferOwnership(*pool.address())
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        println!("Deployed the BetHouse at {}", contract.address());
        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = BetHouse::new(self.address, provider.clone());

        println!("Checking that you are a bettor...");
        let is_bettor =
            contract.isBettor(roles.offender.address()).call().await?;
        Ok(is_bettor)
    }
}
