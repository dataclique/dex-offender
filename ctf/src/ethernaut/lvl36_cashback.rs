//! Ethernaut level 36 — Cashback
//!
//! You've just joined Cashback, the hottest crypto neobank in town. Their pitch
//! is irresistible: for every on-chain payment you make, you earn points. Rack
//! up enough and you'll reach legendary status, unlocking the coveted Super
//! Cashback NFT badge.
//!
//! The system leverages EIP-7702 to allow EOAs to accrue cashback. Users must
//! delegate to the Cashback contract to use the `payWithCashback` function.
//!
//! Rumor has it there's a back door for power users. Your brief is simple:
//! become the loyalty program's nightmare. Max out your cashback in every
//! supported currency and walk away with at least two Super Cashback NFT, one
//! of which must correspond to your player address.

// TODO: implement deployment and the completion check, then declare this
// module in `ctf/src/ethernaut/mod.rs` (add the `level36` field to the
// `Ethernaut` struct and the `use` alias) when you want it deployed.

use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::cashback::Cashback;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level36)
    }

    fn name(&self) -> &'static str {
        "Cashback"
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the Cashback contract...");
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
