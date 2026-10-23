//! Damn Vulnerable DeFi — PuppetV3.
//!
//! Drain the PuppetV3Pool by manipulating the UniswapV3 price.
//!
//! Fork-only: deploys against the real Uniswap V3 factory + WETH +
//! PositionManager. Run deploy_levels with `FORK_URL` (and
//! `FORK_BLOCK=15450164`) set.

use crate::{Level, Roles, anvil, to_ether};
use alloy::{
    primitives::aliases::{I24, U24},
    primitives::{Address, U160, U256, address},
    providers::Provider,
    sol,
};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    DamnValuableToken,
    "abi/damn-vulnerable-defi/DamnValuableToken.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Weth,
    "abi/damn-vulnerable-defi/WETH.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    NonfungiblePositionManager,
    "abi/damn-vulnerable-defi/NonfungiblePositionManager.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UniswapV3Factory,
    "abi/damn-vulnerable-defi/UniswapV3Factory.json"
);
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    PuppetV3Pool,
    "abi/damn-vulnerable-defi/PuppetV3Pool.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
    pub token: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "PuppetV3"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let dvd = ctfs
            .ethereum
            .damn_vulnerable_defi
            .puppet_v3
            .as_ref()
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?;
        Ok(Target { address: dvd.address, token: dvd.token })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let player = &roles.offender;
        let provider = roles.provider();

        // Fork-bound mainnet contracts.
        const MAINNET_WETH: Address =
            address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
        const MAINNET_V3_FACTORY: Address =
            address!("1F98431c8aD98523631AE4a59f267346ea31F984");
        const MAINNET_V3_POSITION_MANAGER: Address =
            address!("C36442b4a4522E871399CD717aBDD847Ab11FE88");

        let weth = Weth::new(MAINNET_WETH, provider.clone());
        let npm = NonfungiblePositionManager::new(
            MAINNET_V3_POSITION_MANAGER,
            provider.clone(),
        );
        let factory =
            UniswapV3Factory::new(MAINNET_V3_FACTORY, provider.clone());

        // Fail fast when this runs on a plain local chain (no fork).
        let factory_code = provider.get_code_at(MAINNET_V3_FACTORY).await?;
        if factory_code.is_empty() {
            return Err(eyre::eyre!(
                "PuppetV3 needs a mainnet fork (set FORK_URL, FORK_BLOCK=15450164)"
            ));
        }

        let send = |c: &str| format!("[puppet-v3] {c}");
        println!("{}", send("wrapping ETH"));
        weth.deposit()
            .value(to_ether(100))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        println!("{}", send("deploying DVT"));
        let token = DamnValuableToken::deploy(provider).await?;

        println!("{}", send("creating v3 pool"));
        let is_weth_first = MAINNET_WETH < *token.address();
        let (token0, token1) = if is_weth_first {
            (MAINNET_WETH, *token.address())
        } else {
            (*token.address(), MAINNET_WETH)
        };
        // sqrtPriceX96 for price 1:1 = sqrt(2^192) = 2^96.
        let sqrt_price_x96: U160 = U160::from(2).pow(U160::from(96));
        npm.createAndInitializePoolIfNecessary(
            token0,
            token1,
            U24::from(3000),
            sqrt_price_x96,
        )
        .from(deployer.address())
        .send()
        .await?
        .watch()
        .await?;

        let pool_addr =
            factory.getPool(token0, token1, U24::from(3000)).call().await?;
        println!("{} {}", send("pool at"), pool_addr);

        println!("{}", send("minting liquidity"));
        weth.approve(*npm.address(), U256::MAX)
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        token
            .approve(*npm.address(), U256::MAX)
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        let deadline = U256::MAX;
        npm.mint(INonfungiblePositionManager::MintParams {
            token0,
            token1,
            fee: U24::from(3000),
            tickLower: I24::try_from(-60).expect("tick"),
            tickUpper: I24::try_from(60).expect("tick"),
            amount0Desired: to_ether(100),
            amount1Desired: to_ether(100),
            amount0Min: U256::ZERO,
            amount1Min: U256::ZERO,
            recipient: deployer.address(),
            deadline,
        })
        .from(deployer.address())
        .send()
        .await?
        .watch()
        .await?;

        println!("{}", send("deploying lending pool"));
        let lending = PuppetV3Pool::deploy(
            provider,
            MAINNET_WETH,
            *token.address(),
            pool_addr,
        )
        .await?;

        token
            .transfer(player.address(), to_ether(110))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        token
            .transfer(*lending.address(), U256::from(1_000_000) * to_ether(1))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        anvil::set_balance(roles, player.address(), to_ether(1)).await?;

        println!("Deployed the PuppetV3 challenge at {}", lending.address());
        Ok(Target { address: *lending.address(), token: *token.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let token = DamnValuableToken::new(self.token, provider.clone());

        println!("Checking that the pool is drained...");
        Ok(token.balanceOf(self.address).call().await?
            == alloy::primitives::U256::ZERO)
    }
}
