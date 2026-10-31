//! Paradigm CTF 2021 — hello.
//!
//! Call the target function on the Hello contract.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    Setup,
    "../lib/paradigm-ctf-2021/babysandbox/public/out/Setup.sol/Setup.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "babysandbox"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let t = ctfs
            .ethereum
            .paradigm_2021
            .babysandbox
            .as_ref()
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?;
        let address = t.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();
        println!("Deploying the babysandbox challenge...");
        let setup = Setup::deploy(provider).await?;
        Ok(Target { address: *setup.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let setup = Setup::new(self.address, provider.clone());
        Ok(setup.isSolved().call().await?)
    }
}
