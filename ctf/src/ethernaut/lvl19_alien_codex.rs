//! Ethernaut level 19 — AlienCodex.
//!
//! Claim ownership of AlienCodex by exploiting array length underflow.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    AlienCodex,
    "abi/ethernaut/AlienCodex.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "AlienCodex"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethernaut.level19.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the AlienCodex contract...");
        let contract = AlienCodex::deploy(provider).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = AlienCodex::new(self.address, provider.clone());

        println!("Checking that you claimed ownership...");
        let owner = contract.owner().call().await?;
        Ok(owner == roles.offender.address())
    }
}
