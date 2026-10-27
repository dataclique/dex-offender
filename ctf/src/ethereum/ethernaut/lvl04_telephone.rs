//! Ethernaut level 04 — Telephone.
//!
//! Goal: Make your offender account the owner of the `Telephone` contract.
//!
//! Source: [local](../../../../lib/ethernaut/contracts/src/levels/Telephone.sol) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/contracts/src/levels/Telephone.sol).
//! Challenge: [local](../../../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/telephone.md) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/client/src/gamedata/en/descriptions/levels/telephone.md).
//! [Factory success check](../../../../lib/ethernaut/contracts/src/levels/TelephoneFactory.sol) · [Rust bindings](crate::ethereum::ethernaut::lvl04_telephone).
//!
//! `target` contains the deployed instance address and any level-specific helpers.
//! `offender` is your player account; sign transactions with `offender.address()`.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Telephone,
    "abi/ethernaut/Telephone.json"
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

        println!("Checking that you became the owner...");
        let owner = contract.owner().call().await?;
        Ok(owner == roles.offender.address())
    }
}
