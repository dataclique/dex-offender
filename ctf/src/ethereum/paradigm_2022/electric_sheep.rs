//! Paradigm CTF 2022 — electric-sheep.
//!
//! The setup deploys with no dependencies; `isSolved()` reads the
//! balance of the real mainnet DREAMERS token (`0x1C4d…`), so the
//! check only behaves on a mainnet fork (run deploy_levels with
//! `FORK_URL` set). Any fork block where the token exists works.

use crate::{Level, Roles};
use alloy::{
    primitives::{Address, U256, address},
    providers::Provider,
    sol,
};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Setup,
    "abi/paradigm-2022/electric-sheep.json"
);

const MAINNET_DREAMERS: Address =
    address!("1C4d5CA50419f94fc952a20DdDCDC4182Ef77cdF");

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "electric-sheep"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let t = ctfs
            .ethereum
            .paradigm_2022
            .electric_sheep
            .as_ref()
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?;
        Ok(Target { address: t.address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        // isSolved() calls the real DREAMERS token; without a fork the
        // call to a codeless address reverts and the check errors.
        let dreamers_code = provider.get_code_at(MAINNET_DREAMERS).await?;
        if dreamers_code.is_empty() {
            return Err(eyre::eyre!(
                "electric-sheep needs a mainnet fork (set FORK_URL)"
            ));
        }

        println!("Deploying the electric-sheep challenge...");
        let address = Setup::deploy_builder(provider)
            .value(U256::ZERO)
            .from(roles.deployer.address())
            .deploy()
            .await?;
        Ok(Target { address })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let setup = Setup::new(self.address, provider.clone());
        Ok(setup.isSolved().call().await?)
    }
}
