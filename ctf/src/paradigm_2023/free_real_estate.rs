//! Paradigm CTF 2023 — Free Real Estate.
//!
//! Drain the airdrop distributor.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Challenge,
    "abi/paradigm-2023/free-real-estate.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Free Real Estate"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs
            .paradigm_2023
            .free_real_estate
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?
            .address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the Free Real Estate challenge...");
        // TODO: deploy MerkleDistributor then Challenge(distributor)
        let contract = Challenge::deploy(provider, Address::ZERO).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = Challenge::new(self.address, provider.clone());

        let solved = contract.getScore().call().await? > 0;
        Ok(solved)
    }
}
