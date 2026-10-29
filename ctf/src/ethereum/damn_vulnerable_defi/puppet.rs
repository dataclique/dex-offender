//! Damn Vulnerable DeFi — Puppet.
//!
//! Drain the PuppetPool by manipulating the UniswapV1 price oracle.

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
    PuppetPool,
    "abi/damn-vulnerable-defi/PuppetPool.json"
);
sol!(
    #[sol(rpc)]
    UniswapV1Factory,
    "abi/damn-vulnerable-defi/UniswapV1Factory.json"
);
sol!(
    #[sol(rpc)]
    UniswapV1Exchange,
    "abi/damn-vulnerable-defi/UniswapV1Exchange.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Puppet"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.damn_vulnerable_defi.puppet.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let offender = &roles.offender;
        let provider = roles.provider();

        println!("Deploying the Puppet challenge...");

        let factory = UniswapV1Factory::deploy(provider).await?;
        let template = UniswapV1Exchange::deploy(provider).await?;
        factory
            .initializeFactory(*template.address())
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

        Ok(Target { address: *pool.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let pool = PuppetPool::new(self.address, provider.clone());
        let token = DamnValuableToken::new(
            pool.token().call().await?,
            provider.clone(),
        );

        Ok(token.balanceOf(self.address).call().await? == U256::ZERO)
    }
}
