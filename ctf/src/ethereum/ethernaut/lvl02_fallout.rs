//! Ethernaut level 02 — Fallout.
//!
//! Goal: Become the owner of `Fallout`, the contract that records users' Ether allocations. The owner must be your offender account.
//!
//! Source: [local](../../../../lib/ethernaut/contracts/src/levels/Fallout.sol) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/contracts/src/levels/Fallout.sol).
//! Challenge: [local](../../../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/fallout.md) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/client/src/gamedata/en/descriptions/levels/fallout.md).
//! [Factory success check](../../../../lib/ethernaut/contracts/src/levels/FalloutFactory.sol) · [Rust bindings](crate::ethereum::ethernaut::lvl02_fallout).
//!
//! `target` contains the deployed instance address and any level-specific helpers.
//! `offender` is your player account; sign transactions with `offender.address()`.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Fallout,
    "abi/ethernaut/Fallout.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Fallout"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level02.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        println!("Deploying the Fallout contract...");
        let contract = Fallout::deploy(provider).await?;

        contract
            .Fal1out()
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        let owner = contract.owner().call().await?;
        assert_eq!(owner, deployer.address());

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = Fallout::new(self.address, provider.clone());

        println!("Checking that you claimed ownership of the contract...");
        let owner = contract.owner().call().await?;
        let is_owner = owner == roles.offender.address();

        Ok(is_owner)
    }
}
