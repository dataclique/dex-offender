//! Damn Vulnerable DeFi — Truster.
//!
//! Use the pool's flash loan callback to steal all tokens.

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
    TrusterLenderPool,
    "../lib/damn-vulnerable-defi/out/TrusterLenderPool.sol/TrusterLenderPool.json"
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
        "Truster"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethereum.damn_vulnerable_defi.truster.clone())
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

        Ok(Target {
            address: *pool.address(),
            token: *token.address(),
            recovery: super::recovery_account(),
        })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let offender = &roles.offender;
        let provider = roles.provider();
        let pool = TrusterLenderPool::new(self.address, provider.clone());
        let token = DamnValuableToken::new(self.token, provider.clone());
        let pool_token = pool.token().call().await?;
        eyre::ensure!(pool_token == self.token, "board token mismatch");

        // Upstream: one player transaction; the pool is empty and the
        // recovery account holds the entire pool balance.
        Ok(token.balanceOf(self.address).call().await? == U256::ZERO
            && provider.get_transaction_count(offender.address()).await? == 1
            && token.balanceOf(self.recovery).call().await?
                == to_ether(1_000_000))
    }
}
