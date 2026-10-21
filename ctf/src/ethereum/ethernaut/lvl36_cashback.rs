//! Ethernaut level 36 — Cashback.
//!
//! Max out cashback in every supported currency and get 2+ Super Cashback
//! NFTs.

use crate::{Level, Roles};
use alloy::{primitives::Address, primitives::U256, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Cashback,
    "abi/ethernaut/Cashback.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Cashback"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level36.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        println!("Deploying the Cashback contract...");
        // Cashback takes (address[], uint256[], uint256[], address)
        // TODO: deploy supported tokens and set up cashback rates
        let contract = Cashback::deploy(
            provider,
            vec![],
            vec![],
            vec![],
            deployer.address(),
        )
        .await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = Cashback::new(self.address, provider.clone());

        println!("Checking that you got 2+ Super Cashback NFTs...");
        // Check balanceOfBatch for ERC1155-style NFT ownership
        // TODO: determine the Super Cashback NFT token ID
        let balance = contract
            .balanceOf(roles.offender.address(), U256::from(1))
            .call()
            .await?;
        Ok(balance >= U256::from(2))
    }
}
