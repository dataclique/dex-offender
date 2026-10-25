//! Paradigm CTF 2022 — hint-finance.
//!
//! The setup swaps 10 ETH through the real mainnet Router01 into three
//! real tokens (all with live WETH pairs) and deposits them into fresh
//! HintFinance vaults. On a mainnet fork nothing needs planting — the
//! router, WETH, pairs and tokens all exist. Set `FORK_URL`.

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
    "abi/paradigm-2022/hint-finance.json"
);

const MAINNET_UNISWAP_V2_ROUTER01: Address =
    address!("f164fC0Ec4E93095b804a4795bBe1e041497b92a");

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "hint-finance"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let t = ctfs
            .ethereum
            .paradigm_2022
            .hint_finance
            .as_ref()
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?;
        Ok(Target { address: t.address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        // The setup routes through the real deprecated Router01; its
        // presence distinguishes a mainnet fork from a plain chain.
        let router_code =
            provider.get_code_at(MAINNET_UNISWAP_V2_ROUTER01).await?;
        if router_code.is_empty() {
            return Err(eyre::eyre!(
                "hint-finance needs a mainnet fork (set FORK_URL)"
            ));
        }

        println!("Deploying the hint-finance challenge...");
        let address = Setup::deploy_builder(provider)
            .value(U256::from(30_000_000_000_000_000_000u128))
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
