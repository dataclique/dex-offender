//! Damn Vulnerable DeFi — Selfie.
//!
//! Use a flash loan to gain voting power and drain the pool via governance.

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, primitives::U256, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    DamnValuableVotes,
    "../lib/damn-vulnerable-defi/out/DamnValuableVotes.sol/DamnValuableVotes.json"
);
sol!(
    #[sol(rpc)]
    SimpleGovernance,
    "../lib/damn-vulnerable-defi/out/SimpleGovernance.sol/SimpleGovernance.json"
);
sol!(
    #[sol(rpc)]
    SelfiePool,
    "../lib/damn-vulnerable-defi/out/SelfiePool.sol/SelfiePool.json"
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
        "Selfie"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethereum.damn_vulnerable_defi.selfie.clone())
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        println!("Deploying the Selfie challenge...");
        let token =
            DamnValuableVotes::deploy(provider, to_ether(2_000_000)).await?;
        let governance =
            SimpleGovernance::deploy(provider, *token.address()).await?;
        let pool = SelfiePool::deploy(
            provider,
            *token.address(),
            *governance.address(),
        )
        .await?;

        token
            .transfer(*pool.address(), to_ether(1_500_000))
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
        let provider = roles.provider();
        let token = DamnValuableVotes::new(self.token, provider.clone());

        // Upstream: the pool is empty and the recovery account holds the
        // entire pool balance.
        Ok(token.balanceOf(self.address).call().await? == U256::ZERO
            && token.balanceOf(self.recovery).call().await?
                == to_ether(1_500_000))
    }
}
