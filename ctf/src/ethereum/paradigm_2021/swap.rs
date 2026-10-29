//! Paradigm CTF 2021 — swap.
//!
//! The setup bootstraps against the deprecated canonical Router01
//! address, four canonical stablecoins (DAI/TUSD/USDC/USDT as
//! MockERC20) and WETH9. Router02 bytecode is planted at the Router01
//! address — `swapExactETHForTokens` is selector-identical — with four
//! seeded WETH pairs providing the swap liquidity.

use crate::{Level, Roles, anvil};
use alloy::{
    primitives::{Address, U256},
    sol,
};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    Setup,
    "abi/paradigm-2021/swap.json"
);

sol!(
    #[sol(rpc)]
    WETH9,
    "abi/paradigm-2021/weth9.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "swap"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs
            .ethereum
            .paradigm_2021
            .swap
            .as_ref()
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?
            .address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();
        anvil::plant_weth9(roles).await?;
        anvil::plant_uniswap_v2_factory(roles).await?;
        anvil::plant_uniswap_v2_router_at(
            roles,
            anvil::MAINNET_UNISWAP_V2_ROUTER01,
        )
        .await?;
        for token in [
            anvil::MAINNET_DAI,
            anvil::MAINNET_TUSD,
            anvil::MAINNET_USDC,
            anvil::MAINNET_USDT,
        ] {
            anvil::plant_mock_erc20(roles, token).await?;
        }

        // Four seeded pairs (200k tokens / 200 WETH each) plus the
        // setup's four 100-ETH router swaps.
        let weth = WETH9::new(anvil::MAINNET_WETH9, provider.clone());
        let deployer = roles.deployer.address();
        weth.deposit()
            .value(U256::from(800_000_000_000_000_000_000u128))
            .from(deployer)
            .send()
            .await?
            .get_receipt()
            .await?;
        for token in [
            anvil::MAINNET_DAI,
            anvil::MAINNET_TUSD,
            anvil::MAINNET_USDC,
            anvil::MAINNET_USDT,
        ] {
            anvil::seed_weth_pair(roles, token, 200_000, 200).await?;
        }

        println!("Deploying the swap challenge...");
        let address = Setup::deploy_builder(provider)
            .value(U256::from(400_000_000_000_000_000_000u128))
            .from(deployer)
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
