//! Ethernaut level 33 — Magic Animal Carousel
//!
//! Welcome, dear Anon, to the Magic Carousel, where creatures spin and twirl in
//! a boundless spell. In this magical, infinite digital wheel, they loop and
//! whirl with enchanting zeal.
//!
//! Add a creature to join the fun, but heed the rule, or the game's undone. If
//! an animal joins the ride, take care when you check again, that same animal
//! must be there!
//!
//! Can you break the magic rule of the carousel?

// TODO: implement deployment and the completion check, then declare this
// module in `ctf/src/ethernaut/mod.rs` (add the `level33` field to the
// `Ethernaut` struct and the `use` alias) when you want it deployed.

use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::magic_animal_carousel::MagicAnimalCarousel;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level33)
    }

    fn name(&self) -> &'static str {
        "Magic Animal Carousel"
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the MagicAnimalCarousel contract...");
        // TODO: deploy the level contracts and fund the offender.
        let _ = deployer;
        let target = Target { address: Address::zero() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        // TODO: check the completion condition for this level.
        let _ = roles;
        Ok(false)
    }
}
