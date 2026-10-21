//! Paradigm CTF 2023 — Dodont.
//!
//! Exploit the donation matching system.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Challenge,
    "abi/paradigm-2023/dodont.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Dodont"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs
            .paradigm_2023
            .dodont
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?
            .address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the Dodont challenge...");
        // TODO: deploy DamnValuableToken then Challenge(dvm)
        let contract = Challenge::deploy(provider, Address::ZERO).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = Challenge::new(self.address, provider.clone());

        let solved = contract.isSolved().call().await?;
        Ok(solved)
    }
}
