//! Ethernaut level 24 — PuzzleWallet.
//!
//! Become the owner and drain the PuzzleWallet proxy.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    PuzzleWallet,
    "abi/ethernaut/PuzzleWallet.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "PuzzleWallet"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = Address::ZERO; // TODO: wire into Ethernaut struct
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the PuzzleWallet contract...");
        // TODO: deploy PuzzleWallet behind a PuzzleProxy
        let contract = PuzzleWallet::deploy(provider).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();

        println!("Checking that you became the owner...");
        let contract = PuzzleWallet::new(self.address, provider.clone());
        let owner = contract.owner().call().await?;
        Ok(owner == roles.offender.address())
    }
}
