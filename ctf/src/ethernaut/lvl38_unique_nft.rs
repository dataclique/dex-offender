//! Ethernaut level 38 — UniqueNFT
//!
//! Welcome to UniqueNFT, where any user can get its very own shiny digital
//! badge.
//!
//! Humans with EOAs? You mint for free, no questions asked. Proof of Humanity
//! for the win!
//!
//! Smart contracts? Sorry bots, pay the toll –> one whole ether!
//!
//! But here's the twist: one badge per address, no greedy hoarding allowed.
//!
//! And forget about trading, these things stick like glue. It's like blockchain
//! tattoos, once it's yours, it's yours forever.
//!
//! Think you can outsmart the rules and own more a single NFT? Prove it.

// TODO: implement deployment and the completion check, then declare this
// module in `ctf/src/ethernaut/mod.rs` (add the `level38` field to the
// `Ethernaut` struct and the `use` alias) when you want it deployed.

use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::unique_nft::UniqueNFT;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level38)
    }

    fn name(&self) -> &'static str {
        "UniqueNFT"
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the UniqueNFT contract...");
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
