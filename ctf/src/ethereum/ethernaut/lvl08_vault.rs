//! Ethernaut level 08 — Vault.
//!
//! Goal: Unlock the password-protected `Vault` so that its `locked` flag becomes false.
//!
//! Source: [local](../../../../lib/ethernaut/contracts/src/levels/Vault.sol) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/contracts/src/levels/Vault.sol).
//! Challenge: [local](../../../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/vault.md) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/client/src/gamedata/en/descriptions/levels/vault.md).
//! [Factory success check](../../../../lib/ethernaut/contracts/src/levels/VaultFactory.sol) · [Rust bindings](crate::ethereum::ethernaut::lvl08_vault).
//!
//! `target` contains the deployed instance address and any level-specific helpers.
//! `offender` is your player account; sign transactions with `offender.address()`.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Vault,
    "abi/ethernaut/Vault.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Vault"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level08.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        println!("Deploying the Vault contract...");
        let random = rand::random::<[u8; 32]>();
        let psswd = alloy::primitives::keccak256(random);
        let contract = Vault::deploy(provider, psswd).await?;

        let _ = deployer;
        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = Vault::new(self.address, provider.clone());

        println!("Checking that the contract is unlocked...");
        let locked = contract.locked().call().await?;
        Ok(!locked)
    }
}
