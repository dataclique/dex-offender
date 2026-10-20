//! Ethernaut level 32 — Impersonator
//!
//! SlockDotIt's new product, ECLocker, integrates IoT gate locks with Solidity
//! smart contracts, utilizing Ethereum ECDSA for authorization. When a valid
//! signature is sent to the lock, the system emits an `Open` event, unlocking
//! doors for the authorized controller. SlockDotIt has hired you to assess the
//! security of this product before its launch. Can you compromise the system in
//! a way that anyone can open the door?

// TODO: implement deployment and the completion check, then declare this
// module in `ctf/src/ethernaut/mod.rs` (add the `level32` field to the
// `Ethernaut` struct and the `use` alias) when you want it deployed.

use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::impersonator::Impersonator;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level32)
    }

    fn name(&self) -> &'static str {
        "Impersonator"
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the Impersonator contract...");
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
