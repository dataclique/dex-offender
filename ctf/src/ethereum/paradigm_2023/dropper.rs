//! Paradigm CTF 2023 — Dropper.
//!
//! Claim the airdrop improperly.

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    Challenge,
    "abi/paradigm-2023/dropper.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Dropper"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs
            .ethereum
            .paradigm_2023
            .dropper
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?
            .address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        // Mirror the upstream Deploy script: bare deploy then fund the
        // escrow with 500 ether.
        let addr = Challenge::deploy_builder(provider)
            .from(deployer.address())
            .deploy()
            .await?;
        let contract = Challenge::new(addr, provider.clone());
        contract
            .deposit()
            .value(to_ether(500))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        Ok(Target { address: addr })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = Challenge::new(self.address, provider.clone());

        let solved = contract.getScore().call().await? > 0;
        Ok(solved)
    }
}
