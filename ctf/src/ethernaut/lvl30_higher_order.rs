//! Ethernaut level 30 — HigherOrder
//!
//! Imagine a world where the rules are meant to be broken, and only the cunning
//! and the bold can rise to power. Welcome to the Higher Order, a group shrouded
//! in mystery, where a treasure awaits and a commander rules supreme.
//!
//! Your objective is to become the Commander of the Higher Order! Good luck!
//!
//! Things that might help:
//! - Sometimes, `calldata` cannot be trusted.
//! - Compilers are constantly evolving into better spaceships.

// TODO: implement deployment and the completion check, then declare this
// module in `ctf/src/ethernaut/mod.rs` (add the `level30` field to the
// `Ethernaut` struct and the `use` alias) when you want it deployed.

use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::higher_order::HigherOrder;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level30)
    }

    fn name(&self) -> &'static str {
        "HigherOrder"
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the HigherOrder contract...");
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
