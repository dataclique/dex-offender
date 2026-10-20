//! Ethernaut level 40 — NotOptimisticPortal
//!
//! This portal relies on a complex chain of cryptographic proofs to verify
//! cross-chain messages. It claims to be secure against invalid state
//! transitions, but the gap between verification and execution might be wider
//! than it looks.
//!
//! Can you manage to mint some tokens for your wallet?
//!
//! Things that might help:
//! - Understanding Function Selectors.
//! - The Checks-Effects-Interactions (CEI) pattern.
//! - Merkle Patricia Tries and RLP encoding.
//!
//! Tips:
//! - Sometimes the data you verify isn't exactly the same data you execute.
//! - If a hash cycle seems impossible to solve, look for a way to break the
//!   loop.

// TODO: implement deployment and the completion check, then declare this
// module in `ctf/src/ethernaut/mod.rs` (add the `level40` field to the
// `Ethernaut` struct and the `use` alias) when you want it deployed.

use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::not_optimistic_portal::NotOptimisticPortal;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level40)
    }

    fn name(&self) -> &'static str {
        "NotOptimisticPortal"
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the NotOptimisticPortal contract...");
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
