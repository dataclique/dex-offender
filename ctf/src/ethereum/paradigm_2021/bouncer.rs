//! Paradigm CTF 2021 — bouncer.
//!
//! Call the target function on the Hello contract.

use crate::{Level, Roles, anvil::plant_weth9, to_ether};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    Setup,
    "../lib/paradigm-ctf-2021/bouncer/public/out/Setup.sol/Setup.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "bouncer"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs
            .ethereum
            .paradigm_2021
            .bouncer
            .as_ref()
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?
            .address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();
        println!("Deploying the bouncer challenge...");
        // The setup wraps the canonical mainnet WETH9 — plant it first.
        plant_weth9(roles).await?;
        let setup = Setup::deploy_builder(provider)
            .value(to_ether(100))
            .from(roles.deployer.address())
            .deploy()
            .await?;
        Ok(Target { address: setup })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let setup = Setup::new(self.address, provider.clone());
        Ok(setup.isSolved().call().await?)
    }
}
