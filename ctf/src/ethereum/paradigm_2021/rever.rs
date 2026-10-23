//! Paradigm CTF 2021 — hello.
//!
//! Call the target function on the Hello contract.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Setup,
    "abi/paradigm-2021/rever.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "rever"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let t = ctfs
            .ethereum
            .paradigm_2021
            .rever
            .as_ref()
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?;
        let address = t.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();
        println!("Deploying the rever challenge...");
        let setup = Setup::deploy(provider).await?;
        Ok(Target { address: *setup.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let _ = roles;
        // Rever is judged by calling `test(string)` with the player's
        // hand-written bytecode (manual judgement; no auto isSolved).
        Ok(false)
    }
}
