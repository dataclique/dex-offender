//! Ethernaut level 39 — Forger.
//!
//! Make the total supply greater than 100 tokens.

use crate::{Level, Roles};
use alloy::{primitives::Address, primitives::U256, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Forger,
    "abi/ethernaut/Forger.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Forger"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethernaut.level39.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        println!("Deploying the Forger contract...");
        let contract = Forger::deploy(provider).await?;

        // The golden signature mints 100 tokens initially
        // TODO: set up the golden signature scenario from the level description
        let _ = deployer;
        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = Forger::new(self.address, provider.clone());

        println!("Checking that total supply > 100...");
        let supply = contract.totalSupply().call().await?;
        Ok(supply > U256::from(100))
    }
}
