//! Paradigm CTF 2021 — farmer.
//!
//! The setup bootstraps against canonical mainnet COMP/DAI tokens, the
//! UniswapV2 Router02 and WETH9. Locally these are planted as mocks
//! (MockERC20 for COMP/DAI) plus the real Router02 bytecode, with two
//! seeded WETH pairs providing the swap liquidity the setup needs.

use crate::{Level, Roles, anvil};
use alloy::{
    primitives::{Address, U256},
    providers::Provider,
    sol,
    sol_types::SolValue,
};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Setup,
    "abi/paradigm-2021/farmer.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Factory,
    "abi/damn-vulnerable-defi/UniswapV2Factory.json"
);

sol!(
    #[allow(missing_docs)]
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

        seed_pair(roles, anvil::MAINNET_COMP, 2_000, 200).await?;
        seed_pair(roles, anvil::MAINNET_DAI, 400_000, 200).await?;

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

/// Create the WETH/`token` pair and give it `tokens` of mock-token
/// balance (via storage seeding) and `weth_amount` of real WETH, then
/// mint LP tokens so the pair has reserves.
async fn seed_pair(
    roles: &Roles,
    token: Address,
    tokens: u128,
    weth_amount: u128,
) -> eyre::Result<()> {
    let deployer = roles.deployer.address();
    let provider = roles.provider();
    let factory =
        Factory::new(anvil::MAINNET_UNISWAP_V2_FACTORY, provider.clone());
    let weth = WETH9::new(anvil::MAINNET_WETH9, provider.clone());

    factory
        .createPair(anvil::MAINNET_WETH9, token)
        .from(deployer)
        .send()
        .await?
        .get_receipt()
        .await?;
    let pair_address =
        factory.getPair(anvil::MAINNET_WETH9, token).call().await?;
    // artifact types the pair return as bytes20
    let pair_address = Address::new(pair_address.0.0);
    if pair_address.is_zero() {
        return Err(eyre::eyre!("createPair produced no pair"));
    }

    anvil::seed_erc20_balance(
        roles,
        token,
        pair_address,
        U256::from(tokens) * U256::from(1_000_000_000_000_000_000u128),
    )
    .await?;

    weth.transfer(
        pair_address,
        U256::from(weth_amount) * U256::from(1_000_000_000_000_000_000u128),
    )
    .from(deployer)
    .send()
    .await?
    .get_receipt()
    .await?;

    // pair.mint(to): sync reserves from balances and mint LP
    let mut mint_calldata = alloy::primitives::hex!("6a627842").to_vec();
    mint_calldata.extend_from_slice(&deployer.abi_encode());
    provider
        .send_transaction(
            alloy::rpc::types::TransactionRequest::default()
                .to(pair_address)
                .from(deployer)
                .input(mint_calldata.into()),
        )
        .await?
        .get_receipt()
        .await?;
    Ok(())
}
