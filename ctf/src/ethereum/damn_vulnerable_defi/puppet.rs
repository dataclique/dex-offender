//! Damn Vulnerable DeFi — Puppet.
//!
//! Drain the PuppetPool by manipulating the UniswapV1 price oracle.

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, primitives::U256, providers::Provider, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    DamnValuableToken,
    "../lib/damn-vulnerable-defi/out/DamnValuableToken.sol/DamnValuableToken.json"
);
sol!(
    #[sol(rpc)]
    PuppetPool,
    "../lib/damn-vulnerable-defi/out/PuppetPool.sol/PuppetPool.json"
);
sol!(
    #[sol(rpc)]
    UniswapV1Factory,
    "../lib/damn-vulnerable-defi/out/IUniswapV1Factory.sol/IUniswapV1Factory.json"
);
sol!(
    #[sol(rpc)]
    UniswapV1Exchange,
    "../lib/damn-vulnerable-defi/out/IUniswapV1Exchange.sol/IUniswapV1Exchange.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
    pub token: Address,
    /// The designated recovery account.
    pub recovery: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Puppet"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethereum.damn_vulnerable_defi.puppet.clone())
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let offender = &roles.offender;
        let provider = roles.provider();

        println!("Deploying the Puppet challenge...");

        // Upstream deploys the pinned Uniswap V1 bytecode (deployCode);
        // the factory binding covers the typed calls.
        let (factory_address, template) =
            crate::uniswap::deploy_v1(roles).await?;
        let factory = UniswapV1Factory::new(factory_address, provider.clone());
        factory
            .initializeFactory(template)
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        let token = DamnValuableToken::deploy(provider).await?;
        factory
            .createExchange(*token.address())
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        let exchange = factory.getExchange(*token.address()).call().await?;

        token
            .approve(exchange, to_ether(10))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        UniswapV1Exchange::new(exchange, provider.clone())
            .addLiquidity(U256::ZERO, to_ether(10), U256::MAX)
            .value(to_ether(10))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        let pool =
            PuppetPool::deploy(provider, *token.address(), exchange).await?;
        token
            .transfer(*pool.address(), to_ether(100_000))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        token
            .transfer(offender.address(), to_ether(1000))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        Ok(Target {
            address: *pool.address(),
            token: *token.address(),
            recovery: super::recovery_account(),
        })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let offender = &roles.offender;
        let provider = roles.provider();
        let token = DamnValuableToken::new(self.token, provider.clone());

        // Upstream: a single player transaction; the pool is empty and the
        // recovery account holds at least the initial pool balance.
        Ok(token.balanceOf(self.address).call().await? == U256::ZERO
            && provider.get_transaction_count(offender.address()).await? == 1
            && token.balanceOf(self.recovery).call().await?
                >= to_ether(100_000))
    }
}
