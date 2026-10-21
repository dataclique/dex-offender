//! Ethernaut level 38 — UniqueNFT.
//!
//! Own more than one NFT to prove you can outsmart the rules.

use crate::{Level, Roles};
use alloy::{primitives::Address, primitives::U256, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UniqueNFT,
    "abi/ethernaut/UniqueNFT.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "UniqueNFT"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethernaut.level38.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the UniqueNFT contract...");
        let contract = UniqueNFT::deploy(provider).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = UniqueNFT::new(self.address, provider.clone());

        println!("Checking that you own more than one NFT...");
        let balance =
            contract.balanceOf(roles.offender.address()).call().await?;
        Ok(balance > U256::from(1))
    }
}
