//! Ethernaut level 37 — Impersonator Two.
//!
//! Steal all the funds from the contract.

use crate::{Level, Roles};
use alloy::{primitives::Address, primitives::U256, providers::Provider, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ImpersonatorTwo,
    "abi/ethernaut/ImpersonatorTwo.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Impersonator Two"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level37.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        println!("Deploying the ImpersonatorTwo contract...");
        let contract = ImpersonatorTwo::deploy(provider).await?;

        // Fund the contract
        deployer.transfer(*contract.address(), to_ether(1)).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();

        println!("Checking that all funds are stolen...");
        let balance = provider.get_balance(self.address).await?;
        Ok(balance == U256::ZERO)
    }
}

fn to_ether(amount: u64) -> U256 {
    U256::from(amount) * U256::from(10).pow(U256::from(18))
}
