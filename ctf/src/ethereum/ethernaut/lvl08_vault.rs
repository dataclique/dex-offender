//! Ethernaut 08 — Vault.
//!
//! Goal: Unlock the password-protected `Vault` so that its `locked` flag
//! becomes false.
//!
//! Source: [Vault.sol][source]. [Rust harness][harness].
//!
//! [source]:
//! ../../../../lib/ethernaut/contracts/src/levels/Vault.sol
//! [harness]: crate::ethereum::ethernaut::lvl08_vault

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
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
