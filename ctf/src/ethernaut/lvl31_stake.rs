//! Ethernaut level 31 — Stake
//!
//! Stake is safe for staking native ETH and ERC20 WETH, considering the same
//! 1:1 value of the tokens. Can you drain the contract?
//!
//! To complete this level, the contract state must meet the following
//! conditions:
//!
//! - The `Stake` contract's ETH balance has to be greater than 0.
//! - `totalStaked` must be greater than the `Stake` contract's ETH balance.
//! - You must be a staker.
//! - Your staked balance must be 0.

// TODO: implement deployment and the completion check, then declare this
// module in `ctf/src/ethernaut/mod.rs` (add the `level31` field to the
// `Ethernaut` struct and the `use` alias) when you want it deployed.

use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::stake::Stake;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level31)
    }

    fn name(&self) -> &'static str {
        "Stake"
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the Stake contract...");
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
