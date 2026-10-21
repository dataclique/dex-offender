//! Ethernaut level 32 — Impersonator.
//!
//! Compromise the ECLocker so anyone can open the door.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Impersonator,
    "abi/ethernaut/Impersonator.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Impersonator"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level32.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the Impersonator contract...");
        let contract =
            Impersonator::deploy(provider, alloy::primitives::U256::from(1))
                .await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = Impersonator::new(self.address, provider.clone());

        println!("Checking that the lock is compromised...");
        // Solved when anyone (not just authorized) can open the door
        // TODO: determine exact check from the contract source
        let owner = contract.owner().call().await?;
        Ok(owner == roles.offender.address())
    }
}
