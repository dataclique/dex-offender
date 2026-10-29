//! Paradigm CTF 2022 — stealing-sats.
//!
//! The setup snapshots the WBTC balance of a fixed mainnet address
//! (`0x8D90…`) at deploy time. The address held ~0.29 WBTC during the
//! challenge's 2022 window but holds 0 at HEAD — fork with
//! `FORK_BLOCK=15707136` (verified: 28,757,013 sats) for faithful
//! semantics; at HEAD the setup still deploys and stays unsolved.

use crate::{Level, Roles};
use alloy::{
    primitives::{Address, address},
    providers::Provider,
    sol,
};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    Setup,
    "abi/paradigm-2022/stealing-sats.json"
);

const MAINNET_WBTC: Address =
    address!("2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599");

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "stealing-sats"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let t = ctfs
            .ethereum
            .paradigm_2022
            .stealing_sats
            .as_ref()
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?;
        Ok(Target { address: t.address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        // The constructor reads the real WBTC contract; its presence
        // distinguishes a mainnet fork from a plain chain.
        let wbtc_code = provider.get_code_at(MAINNET_WBTC).await?;
        if wbtc_code.is_empty() {
            return Err(eyre::eyre!(
                "stealing-sats needs a mainnet fork (set FORK_URL)"
            ));
        }

        println!("Deploying the stealing-sats challenge...");
        let address = Setup::deploy_builder(provider)
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
