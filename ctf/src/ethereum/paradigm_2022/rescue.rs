//! Paradigm CTF 2022 — rescue.
//!
//! The setup strands 10 WETH inside a MasterChefHelper whose exploit
//! path routes through the real SushiSwap MasterChef and SushiRouter
//! (both live on mainnet). On a mainnet fork nothing needs planting;
//! set `FORK_URL`.

use crate::{Level, Roles};
use alloy::{
    primitives::{Address, U256, address},
    providers::Provider,
    sol,
};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    Setup,
    "../lib/paradigm-ctf-2022/rescue/public/out/Setup.sol/Setup.json"
);

const MAINNET_SUSHISWAP_MASTERCHEF: Address =
    address!("c2EdaD668740f1aA35E4D8f227fB8E17dcA888Cd");

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "rescue"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let t = ctfs
            .ethereum
            .paradigm_2022
            .rescue
            .as_ref()
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?;
        Ok(Target { address: t.address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        // The helper's constants point at the real SushiSwap
        // MasterChef; its presence distinguishes a mainnet fork.
        let chef_code =
            provider.get_code_at(MAINNET_SUSHISWAP_MASTERCHEF).await?;
        if chef_code.is_empty() {
            return Err(eyre::eyre!(
                "rescue needs a mainnet fork (set FORK_URL)"
            ));
        }

        println!("Deploying the rescue challenge...");
        let address = Setup::deploy_builder(provider)
            .value(U256::from(10_000_000_000_000_000_000u128))
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
