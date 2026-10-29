//! Ethernaut 33 — Magic Animal Carousel.
//!
//! Goal: Break `MagicAnimalCarousel`'s animal-storage rule: after the checker
//! stores "Goat" and spins, the current crate must not contain the expected
//! animal encoding.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/MagicAnimalCarousel.sol

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
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
        let address = ctfs.ethereum.ethernaut.level33.address;
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
        // Mirror validateInstance: store a goat, spin, then the stored
        // animal must differ from the plain "Goat" encoding.
        contract
            .setAnimalAndSpin("Goat".to_string())
            .from(roles.deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        let current_crate = contract.currentCrateId().call().await?;
        let animal_in_box =
            contract.carousel(current_crate).call().await? >> 176;
        let goat_enc = alloy::primitives::U256::from_be_slice(&[
            b'G', b'o', b'a', b't', 0, 0, 0, 0, 0, 0,
        ]);
        Ok(animal_in_box != goat_enc)
    }
}
