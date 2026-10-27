//! Ethernaut level 07 — Force.
//!
//! Goal: Give the otherwise empty `Force` contract a positive Ether balance.
//!
//! Source: [local](../../../../lib/ethernaut/contracts/src/levels/Force.sol) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/contracts/src/levels/Force.sol).
//! Challenge: [local](../../../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/force.md) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/client/src/gamedata/en/descriptions/levels/force.md).
//! [Factory success check](../../../../lib/ethernaut/contracts/src/levels/ForceFactory.sol) · [Rust bindings](crate::ethereum::ethernaut::lvl07_force).
//!
//! `target` contains the deployed instance address and any level-specific helpers.
//! `offender` is your player account; sign transactions with `offender.address()`.

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
