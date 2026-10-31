//! Paradigm CTF 2021 — farmer.
//!
//! The setup bootstraps against canonical mainnet COMP/DAI tokens, the
//! UniswapV2 Router02 and WETH9. Locally these are planted as mocks
//! (MockERC20 for COMP/DAI) plus the real Router02 bytecode, with two
//! seeded WETH pairs providing the swap liquidity the setup needs.

use crate::{Level, Roles, anvil};
use alloy::{
    primitives::{Address, U256},
    sol,
};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    Setup,
    "../lib/paradigm-ctf-2021/farmer/public/out/Setup.sol/Setup.json"
);

sol!(
    #[sol(rpc)]
    Factory,
    "../lib/damn-vulnerable-defi/out/IUniswapV2Factory.sol/IUniswapV2Factory.json"
);

sol!(
    #[sol(rpc)]
    WETH9,
    "../attack/out/WETH9.sol/WETH9.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "farmer"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs
            .ethereum
            .paradigm_2021
            .farmer
            .as_ref()
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?
            .address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();
        anvil::plant_weth9(roles).await?;
        anvil::plant_uniswap_v2_factory(roles).await?;
        anvil::plant_uniswap_v2_router(roles).await?;
        anvil::plant_mock_erc20(roles, anvil::MAINNET_COMP).await?;
        anvil::plant_mock_erc20(roles, anvil::MAINNET_DAI).await?;

        // Seed the two pairs the setup and `peekYield` route through:
        // WETH/COMP (funds the faucet swap) and WETH/DAI (the final
        // hop of getAmountsOut COMP→WETH→DAI).
        let weth = WETH9::new(anvil::MAINNET_WETH9, provider.clone());
        let deployer = roles.deployer.address();

        weth.deposit()
            .value(U256::from(400_000_000_000_000_000_000u128))
            .from(deployer)
            .send()
            .await?
            .get_receipt()
            .await?;

        anvil::seed_weth_pair(roles, anvil::MAINNET_COMP, 2_000, 200).await?;
        anvil::seed_weth_pair(roles, anvil::MAINNET_DAI, 400_000, 200).await?;

        println!("Deploying the farmer challenge...");
        let address = Setup::deploy_builder(provider)
            .value(U256::from(50_000_000_000_000_000_000u128))
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
