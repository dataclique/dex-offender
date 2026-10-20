//! Ethernaut level 39 — Forger
//!
//! This is the Forger, the token printer your mom warned you about.
//!
//! This ERC-20 hands out mint passes signed by the owner... or so they say.
//!
//! One golden signature already exists, good for 100 shiny tokens.
//!
//! The team insists the pass is single-use and perfectly safe.
//!
//! Your goal? Making the total supply greater than 100 tokens.
//!
//! Get creative, stay sharp, and may your forgeries be legendary.

// TODO: implement deployment and the completion check, then declare this
// module in `ctf/src/ethernaut/mod.rs` (add the `level39` field to the
// `Ethernaut` struct and the `use` alias) when you want it deployed.

use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::forger::Forger;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level39)
    }

    fn name(&self) -> &'static str {
        "Forger"
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the Forger contract...");
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
