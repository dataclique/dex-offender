//! Ethernaut 19 — Alien Codex.
//!
//! Goal: Make your offender account the owner of `AlienCodex`, a contract with
//! a mutable codex array.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/AlienCodex.sol

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
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
        let address = ctfs.ethereum.ethernaut.level19.address;
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
