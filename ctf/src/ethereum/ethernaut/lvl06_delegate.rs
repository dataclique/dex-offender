//! Ethernaut 06 — Delegation.
//!
//! Goal: Take ownership of the `Delegation` instance, which delegatecalls
//! into a companion `Delegate` contract.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Delegation.sol

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    Delegate,
    "../lib/ethernaut/contracts/out/Delegation.sol/Delegate.json"
);

sol!(
    #[sol(rpc)]
    Delegation,
    "../lib/ethernaut/contracts/out/Delegation.sol/Delegation.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub delegation_address: Address,
    /// The `Delegate` implementation the proxy points at.
    pub delegate_address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Delegation"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let t = &ctfs.ethereum.ethernaut.level06;
        Ok(Target {
            delegation_address: t.delegation_address,
            delegate_address: t.delegate_address,
        })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the Delegation challenge...");
        // Mirror DelegationFactory: a Delegate owned by its deployer, then
        // the Delegation proxy pointing at it.
        let delegate =
            Delegate::deploy(provider, roles.deployer.address()).await?;
        let delegation =
            Delegation::deploy(provider, *delegate.address()).await?;

        Ok(Target {
            delegation_address: *delegation.address(),
            delegate_address: *delegate.address(),
        })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        // pwn() executed through Delegation's delegatecall fallback writes
        // the owner slot in Delegation's own storage — exactly what the
        // factory validates.
        let delegation =
            Delegation::new(self.delegation_address, provider.clone());
        Ok(delegation.owner().call().await? == roles.offender.address())
    }
}
