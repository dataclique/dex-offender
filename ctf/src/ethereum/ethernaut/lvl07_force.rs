//! Ethernaut 07 — Force.
//!
//! Goal: Give the otherwise empty `Force` contract a positive Ether balance.
//!
//! Source: [Force.sol][source]. [Rust harness][harness].
//!
//! [source]:
//! ../../../../lib/ethernaut/contracts/src/levels/Force.sol
//! [harness]: crate::ethereum::ethernaut::lvl07_force

use crate::{Level, Roles};
use alloy::{primitives::Address, providers::Provider, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Force,
    "abi/ethernaut/Force.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Force"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level07.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the Force contract...");
        let contract = Force::deploy(provider).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();

        println!("Checking the contract balance...");
        let balance = provider.get_balance(self.address).await?;
        Ok(balance > alloy::primitives::U256::ZERO)
    }
}
