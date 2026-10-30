//! Damn Vulnerable DeFi — Truster.
//!
//! Use the pool's flash loan callback to steal all tokens.

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, primitives::U256, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    DamnValuableToken,
    "../lib/damn-vulnerable-defi/out/DamnValuableToken.sol/DamnValuableToken.json"
);
sol!(
    #[sol(rpc)]
    TrusterLenderPool,
    "../lib/damn-vulnerable-defi/out/TrusterLenderPool.sol/TrusterLenderPool.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Truster"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.damn_vulnerable_defi.truster.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        println!("Deploying the Truster challenge...");
        let token = DamnValuableToken::deploy(provider).await?;
        let pool =
            TrusterLenderPool::deploy(provider, *token.address()).await?;

        token
            .transfer(*pool.address(), to_ether(1_000_000))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        Ok(Target { address: *pool.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let pool = TrusterLenderPool::new(self.address, provider.clone());

        let token_addr = pool.token().call().await?;
        let token = DamnValuableToken::new(token_addr, provider.clone());

        let pool_balance = token.balanceOf(self.address).call().await?;
        let offender_balance =
            token.balanceOf(roles.offender.address()).call().await?;

        // All tokens moved from pool to offender
        Ok(pool_balance == U256::ZERO
            && offender_balance >= to_ether(1_000_000))
    }
}
