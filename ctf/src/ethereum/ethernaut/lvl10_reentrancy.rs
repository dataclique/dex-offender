//! Ethernaut 10 — Re-entrancy.
//!
//! Goal: Empty the Ether held by `Reentrance`, a contract that records
//! donations and lets users withdraw their balances.
//!
//! Source: [Reentrance.sol][source]. [Rust harness][harness].
//!
//! [source]:
//! ../../../../lib/ethernaut/contracts/src/levels/Reentrance.sol
//! [harness]: crate::ethereum::ethernaut::lvl10_reentrancy

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, providers::Provider, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    Reentrance,
    "abi/ethernaut/Reentrance.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Re-entrancy"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level10.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let some_user = &roles.some_user;
        let provider = roles.provider();

        println!("Deploying the Reentrance contract...");
        let contract = Reentrance::deploy(provider).await?;

        deployer.transfer(*contract.address(), to_ether(1)).await?;

        contract
            .donate(some_user.address())
            .value(to_ether(20))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        let contract =
            Reentrance::new(*contract.address(), some_user.provider().clone());
        contract
            .donate(deployer.address())
            .value(to_ether(100))
            .from(some_user.address())
            .send()
            .await?
            .watch()
            .await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();

        let balance = provider.get_balance(self.address).await?;
        Ok(balance == alloy::primitives::U256::ZERO)
    }
}
