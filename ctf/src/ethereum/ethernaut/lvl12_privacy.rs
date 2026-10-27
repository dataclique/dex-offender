//! Ethernaut 12 — Privacy.
//!
//! Goal: Unlock the `Privacy` contract so that its `locked` flag becomes false.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Privacy.sol

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Privacy,
    "abi/ethernaut/Privacy.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Privacy"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level12.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the Privacy contract...");
        let data = [
            alloy::primitives::keccak256(rand::random::<[u8; 32]>()),
            alloy::primitives::keccak256(rand::random::<[u8; 32]>()),
            alloy::primitives::keccak256(rand::random::<[u8; 32]>()),
        ];
        let contract = Privacy::deploy(provider, data).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = Privacy::new(self.address, provider.clone());

        println!("Checking that you unlocked the contract...");
        let locked = contract.locked().call().await?;
        Ok(!locked)
    }
}
