//! Ethernaut level 06 — Delegation.
//!
//! Goal: Take ownership of the `Delegation` instance, which uses a companion `Delegate` contract. The checked owner belongs to `Delegation`, not `Delegate`.
//!
//! Source: [local](../../../../lib/ethernaut/contracts/src/levels/Delegation.sol) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/contracts/src/levels/Delegation.sol).
//! Challenge: [local](../../../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/delegate.md) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/client/src/gamedata/en/descriptions/levels/delegate.md).
//! [Factory success check](../../../../lib/ethernaut/contracts/src/levels/DelegationFactory.sol) · [Rust bindings](crate::ethereum::ethernaut::lvl06_delegate).
//!
//! `target` contains the deployed instance address and any level-specific helpers.
//! `offender` is your player account; sign transactions with `offender.address()`.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Delegate,
    "abi/ethernaut/Delegation.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub delegation_address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Delegation"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let delegation_address =
            ctfs.ethereum.ethernaut.level06.delegation_address;
        Ok(Target { delegation_address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        // Deploy the implementation contract
        // Note: the Delegation artifact includes both contracts
        let delegate = Delegate::deploy(provider, deployer.address()).await?;

        // Deploy the proxy pointing to the implementation
        let delegation =
            Delegate::deploy(provider, *delegate.address()).await?;

        let owner = delegate.owner().call().await?;
        assert_eq!(owner, deployer.address());

        Ok(Target { delegation_address: *delegation.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let delegation =
            Delegate::new(self.delegation_address, provider.clone());

        println!("Checking that you became the owner...");
        let owner = delegation.owner().call().await?;
        Ok(owner == roles.offender.address())
    }
}
