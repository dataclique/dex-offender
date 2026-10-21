//! Ethernaut level 33 — Magic Animal Carousel.
//!
//! Break the magic rule of the carousel.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    MagicAnimalCarousel,
    "abi/ethernaut/MagicAnimalCarousel.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Magic Animal Carousel"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethernaut.level33.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the MagicAnimalCarousel contract...");
        let contract = MagicAnimalCarousel::deploy(provider).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = MagicAnimalCarousel::new(self.address, provider.clone());

        println!("Checking that the carousel rule is broken...");
        // Solved when an animal exists in the carousel but doesn't match
        // what it should be (the "magic rule" is broken)
        // TODO: determine exact check from the contract source
        let _ = contract;
        Ok(false)
    }
}
