//! Ethernaut level 09 — King.
//!
//! Goal: Claim the throne in `King` and prevent the original owner from reclaiming it during the level check.
//!
//! Source: [local](../../../../lib/ethernaut/contracts/src/levels/King.sol) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/contracts/src/levels/King.sol).
//! Challenge: [local](../../../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/king.md) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/client/src/gamedata/en/descriptions/levels/king.md).
//! [Factory success check](../../../../lib/ethernaut/contracts/src/levels/KingFactory.sol) · [Rust bindings](crate::ethereum::ethernaut::lvl09_king).
//!
//! `target` contains the deployed instance address and any level-specific helpers.
//! `offender` is your player account; sign transactions with `offender.address()`.

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    King,
    "abi/ethernaut/King.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "King"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level09.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        println!("Deploying the King contract...");
        let address = King::deploy_builder(provider)
            .value(to_ether(10))
            .from(deployer.address())
            .deploy()
            .await?;

        Ok(Target { address })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let deployer = &roles.deployer;
        let _contract = King::new(self.address, deployer.provider().clone());

        println!("Attempting to reclaim the kingdom...");
        let result = deployer.transfer(self.address, to_ether(10)).await;

        Ok(result.is_err())
    }
}
