//! Ethernaut 04 — Telephone.
//!
//! Goal: Make your offender account the owner of the `Telephone` contract.
//!
//! Source: [Telephone.sol][source]. [Rust harness][harness].
//!
//! [source]:
//! ../../../../lib/ethernaut/contracts/src/levels/Telephone.sol
//! [harness]: crate::ethereum::ethernaut::lvl04_telephone

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    Telephone,
    "../lib/ethernaut/contracts/out/Telephone.sol/Telephone.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Telephone"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level04.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        println!("Deploying the Telephone contract...");
        let contract = Telephone::deploy(provider).await?;

        let owner = contract.owner().call().await?;
        assert_eq!(owner, deployer.address());

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = Telephone::new(self.address, provider.clone());

        let owner = contract.owner().call().await?;
        Ok(owner == roles.offender.address())
    }
}
