//! Ethernaut level 20 — Denial.
//!
//! Prevent the Denial contract owner from withdrawing funds.

use crate::{Level, Roles};
use alloy::{primitives::Address, primitives::U256, providers::Provider, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Denial,
    "abi/ethernaut/Denial.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Denial"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level20.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        println!("Deploying the Denial contract...");
        let contract = Denial::deploy(provider).await?;

        deployer.transfer(*contract.address(), U256::from(1_000_000)).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        println!("Checking that the contract has more than 100 wei...");
        let balance = provider.get_balance(self.address).await?;
        if balance <= U256::from(100) {
            return Ok(false);
        }

        println!("Checking that the owner cannot call withdraw()...");
        let contract = Denial::new(self.address, deployer.provider().clone());
        let result = contract
            .withdraw()
            .from(deployer.address())
            .gas(1_000_000)
            .send()
            .await;

        match result {
            Err(_) => Ok(true),
            Ok(pending) => {
                let receipt = pending.get_receipt().await?;
                Ok(!receipt.status())
            }
        }
    }
}
