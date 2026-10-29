//! Paradigm CTF 2021 — broker.
//!
//! The setup bootstraps a WETH/token UniswapV2 pool at the canonical
//! factory; both WETH9 and the factory are planted at their canonical
//! mainnet addresses before deployment.

use crate::{Level, Roles, anvil};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    Setup,
    "abi/paradigm-2021/broker.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "broker"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs
            .ethereum
            .paradigm_2021
            .broker
            .as_ref()
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?
            .address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();
        anvil::plant_weth9(roles).await?;
        anvil::plant_uniswap_v2_factory(roles).await?;
        anvil::steer_deployer_below(roles, anvil::MAINNET_WETH9).await?;

        println!("Deploying the broker challenge...");
        let address = Setup::deploy_builder(provider)
            .value(alloy::primitives::U256::from(
                50_000_000_000_000_000_000u128,
            ))
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
