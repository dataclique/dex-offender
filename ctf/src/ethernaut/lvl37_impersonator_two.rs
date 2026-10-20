//! Ethernaut level 37 — Impersonator Two
//!
//! The goal of this level is for you to steal all the funds from the contract.
//!
//! Things that might help:
//! - Look carefully at the 2 signatures that the owner of the contract used to
//!   lock it and set the admin.

// TODO: implement deployment and the completion check, then declare this
// module in `ctf/src/ethernaut/mod.rs` (add the `level37` field to the
// `Ethernaut` struct and the `use` alias) when you want it deployed.

use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::impersonator_two::ImpersonatorTwo;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level37)
    }

    fn name(&self) -> &'static str {
        "Impersonator Two"
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the ImpersonatorTwo contract...");
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
