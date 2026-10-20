//! Ethernaut level 01 — Fallback.
//!
//! Take ownership of the Fallback contract and drain its balance: contribute
//! below the threshold, trigger `receive()`, then `withdraw()` as the new
//! owner.

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, primitives::U256, providers::Provider, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Fallback,
    "abi/ethernaut/Fallback.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Fallback"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethernaut.level01.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        println!("Deploying the Fallback contract...");
        let contract = Fallback::deploy(provider).await?;

        let balance = contract.contributions(deployer.address()).call().await?;
        assert_eq!(balance, to_ether(1000));

        let balance =
            contract.contributions(roles.offender.address()).call().await?;
        assert_eq!(balance, U256::ZERO);

        deployer.transfer(*contract.address(), to_ether(5)).await?;

        let contract_balance =
            provider.get_balance(*contract.address()).await?;
        assert_eq!(contract_balance, to_ether(5));

        let owner = contract.owner().call().await?;
        assert_eq!(owner, deployer.address());

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = Fallback::new(self.address, provider.clone());

        println!("Checking that you claimed ownership of the contract...");
        let owner = contract.owner().call().await?;
        let is_owner = owner == roles.offender.address();

        println!("Checking that you reduced its balance to 0...");
        let contract_balance = provider.get_balance(self.address).await?;
        let balance_reduced = contract_balance == U256::ZERO;

        Ok(is_owner && balance_reduced)
    }
}
