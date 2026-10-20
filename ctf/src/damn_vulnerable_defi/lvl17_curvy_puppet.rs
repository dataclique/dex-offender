//! Damn Vulnerable DeFi level 17 — Curvy Puppet
//!
//! There's a lending contract where anyone can borrow LP tokens from Curve's
//! stETH/ETH pool. To do so, borrowers must first deposit enough Damn Valuable
//! tokens (DVT) as collateral. If a position's borrowed value grows larger than
//! the collateral's value, anyone can liquidate it by repaying the debt and
//! seizing all collateral.
//!
//! Before user funds are taken, close all positions and save all available
//! collateral.
//!
//! NOTE: this challenge requires a valid RPC URL to fork mainnet state into the
//! local environment.
//!
//! Full description: lib/damn-vulnerable-defi/src/curvy-puppet/README.md
//! (pinned submodule, v4.1.0).

// TODO: implement deployment and the completion check, then declare this
// module in `ctf/src/damn_vulnerable_defi/mod.rs` and re-enable the
// `damn_vulnerable_defi` module in `ctf/src/lib.rs` when you want it deployed.

use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::curvy_puppet_lending::CurvyPuppetLending;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.damn_vulnerable_defi.level17)
    }

    fn name(&self) -> &'static str {
        "Curvy Puppet"
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the CurvyPuppetLending contract...");
        // TODO: deploy the level contracts and fund the offender.
        // Requires a mainnet-fork RPC URL (Curve stETH/ETH pool state).
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
