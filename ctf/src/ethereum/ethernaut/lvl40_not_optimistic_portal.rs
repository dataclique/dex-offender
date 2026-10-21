//! Ethernaut level 40 — NotOptimisticPortal.
//!
//! Mint some tokens for your wallet through the cross-chain portal.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    NotOptimisticPortal,
    "abi/ethernaut/NotOptimisticPortal.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "NotOptimisticPortal"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level40.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        println!("Deploying the NotOptimisticPortal contract...");
        // Takes (string, string, bytes, address) — message, version, data, target
        // TODO: determine proper constructor args from the level source
        let contract = NotOptimisticPortal::deploy(
            provider,
            String::new(),
            String::new(),
            alloy::primitives::Bytes::new(),
            deployer.address(),
        )
        .await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = NotOptimisticPortal::new(self.address, provider.clone());

        println!("Checking that you minted tokens...");
        // TODO: determine exact check from the contract source
        // Likely checking a token balance of the offender
        let _ = contract;
        Ok(false)
    }
}
