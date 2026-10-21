//! Ethernaut level 04 — Telephone.
//!
//! Exploit tx.origin vs msg.sender to claim ownership.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Telephone,
    "abi/ethernaut/Telephone.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Telephone"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level04.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        println!("Deploying the Telephone contract...");
        let contract = Telephone::deploy(provider).await?;

        let owner = contract.owner().call().await?;
        assert_eq!(owner, deployer.address());

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = Telephone::new(self.address, provider.clone());

        println!("Checking that you became the owner...");
        let owner = contract.owner().call().await?;
        Ok(owner == roles.offender.address())
    }
}
