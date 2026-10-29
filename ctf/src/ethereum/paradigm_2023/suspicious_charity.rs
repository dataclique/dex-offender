//! Paradigm CTF 2023 — Suspicious Charity.
//!
//! Drain the charity router.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    Router,
    "abi/paradigm-2023/Router.json"
);

sol!(
    #[sol(rpc)]
    Challenge,
    "abi/paradigm-2023/suspicious-charity.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Suspicious Charity"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs
            .ethereum
            .paradigm_2023
            .suspicious_charity
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?
            .address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        // Mirror the upstream Deploy script: Router then Challenge(router).
        let router = Router::deploy(provider).await?;
        let contract = Challenge::deploy(provider, *router.address()).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let _contract = Challenge::new(self.address, provider.clone());

        // TODO: determine completion condition (no isSolved or getScore)
        Ok(false)
    }
}
