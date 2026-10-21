//! Damn Vulnerable DeFi level 16 — Shards
//!
//! The Shards NFT marketplace is a permissionless smart contract enabling
//! holders of Damn Valuable NFTs to sell them at any price (expressed in USDC).
//!
//! These NFTs could be so damn valuable that sellers can offer them in smaller
//! fractions ("shards"). Buyers can buy these shards, represented by an ERC1155
//! token. The marketplace only pays the seller once the whole NFT is sold.
//!
//! Full description: lib/damn-vulnerable-defi/src/shards/README.md (pinned
//! submodule, v4.1.0).

// TODO: implement deployment and the completion check, then declare this
// module in `ctf/src/damn_vulnerable_defi/mod.rs` and re-enable the
// `damn_vulnerable_defi` module in `ctf/src/lib.rs` when you want it deployed.

use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::shards_nft_marketplace::ShardsNFTMarketplace;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethereum.damn_vulnerable_defi.level16)
    }

    fn name(&self) -> &'static str {
        "Shards"
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the ShardsNFTMarketplace contract...");
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
