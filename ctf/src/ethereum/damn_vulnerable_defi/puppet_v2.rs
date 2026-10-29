//! Damn Vulnerable DeFi — PuppetV2.
//!
//! Drain the PuppetV2Pool by manipulating the UniswapV2 price.

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, primitives::U256, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    DamnValuableToken,
    "abi/damn-vulnerable-defi/DamnValuableToken.json"
);
sol!(
    #[sol(rpc)]
    WETH,
    "abi/damn-vulnerable-defi/WETH.json"
);
sol!(
    #[sol(rpc)]
    PuppetV2Pool,
    "abi/damn-vulnerable-defi/PuppetV2Pool.json"
);
sol!(
    #[sol(rpc)]
    UniswapV2Factory,
    "abi/damn-vulnerable-defi/UniswapV2Factory.json"
);
sol!(
    #[sol(rpc)]
    UniswapV2Router02,
    "abi/damn-vulnerable-defi/UniswapV2Router02.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
    pub token: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "PuppetV2"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let dvd = &ctfs.ethereum.damn_vulnerable_defi.puppet_v2;
        Ok(Target { address: dvd.address, token: dvd.token })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let offender = &roles.offender;
        let provider = roles.provider();

        println!("Deploying the PuppetV2 challenge...");

        let token = DamnValuableToken::deploy(provider).await?;
        let weth = WETH::deploy(provider).await?;

        // Deploy UniswapV2 factory (feeToSetter = address(0))
        let factory = UniswapV2Factory::deploy(provider, Address::ZERO).await?;
        // Deploy router
        let router = UniswapV2Router02::deploy(
            provider,
            *factory.address(),
            *weth.address(),
        )
        .await?;

        // Add liquidity: 100 token + 10 WETH
        token
            .approve(*router.address(), to_ether(100))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        router
            .addLiquidityETH(
                *token.address(),
                to_ether(100),
                U256::ZERO,
                U256::ZERO,
                deployer.address(),
                U256::MAX,
            )
            .value(to_ether(10))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        let pair =
            factory.getPair(*token.address(), *weth.address()).call().await?;

        // Deploy lending pool
        let pool = PuppetV2Pool::deploy(
            provider,
            *weth.address(),
            *token.address(),
            pair,
            *factory.address(),
        )
        .await?;

        // Fund pool + offender
        token
            .transfer(*pool.address(), to_ether(1_000_000))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        token
            .transfer(offender.address(), to_ether(10_000))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        Ok(Target { address: *pool.address(), token: *token.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let token = DamnValuableToken::new(self.token, provider.clone());

        println!("Checking that the pool is drained...");
        Ok(token.balanceOf(self.address).call().await? == U256::ZERO)
    }
}
