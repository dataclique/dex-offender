//! Ethernaut level 25 — Motorbike.
//!
//! Destroy the Motorbike engine by selfdestructing its implementation.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Motorbike,
    "abi/ethernaut/Motorbike.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Motorbike"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level25.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the Motorbike contract...");
        // Motorbike takes the engine address as constructor arg.
        // The Engine is deployed first in the upstream factory, then passed.
        // For now we deploy with a placeholder — the actual Engine ABI
        // needs to be extracted separately.
        // TODO: extract Engine ABI and deploy it first
        let contract = Motorbike::deploy(provider, Address::ZERO).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();

        println!("Checking that the engine is destroyed...");
        // Solved when the engine's implementation has no code
        // (extcodesize == 0 after selfdestruct)
        // The engine address is stored in the Motorbike contract
        let contract = Motorbike::new(self.address, provider.clone());
        // TODO: read the engine address from Motorbike's storage and check
        // that its code size is 0 (selfdestructed)
        let _ = contract;
        Ok(false)
    }
}
