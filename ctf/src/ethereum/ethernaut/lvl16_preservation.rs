//! Ethernaut 16 — Preservation.
//!
//! Goal: Take ownership of `Preservation`, a timezone-recording contract that
//! uses two `LibraryContract` instances.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Preservation.sol

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    Preservation,
    "abi/ethernaut/Preservation.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Preservation"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level16.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the Preservation contract...");
        // TODO: deploy LibraryContract instances and pass to Preservation
        let contract = Preservation::deploy(
            provider,
            alloy::primitives::Address::ZERO,
            alloy::primitives::Address::ZERO,
        )
        .await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = Preservation::new(self.address, provider.clone());

        println!("Checking that you claimed ownership of the contract...");
        let owner = contract.owner().call().await?;
        Ok(owner == roles.offender.address())
    }
}
