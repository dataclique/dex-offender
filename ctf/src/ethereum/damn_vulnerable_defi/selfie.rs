//! Damn Vulnerable DeFi — Selfie.
//!
//! Use a flash loan to gain voting power and drain the pool via governance.

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, primitives::U256, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    DamnValuableVotes,
    "abi/damn-vulnerable-defi/DamnValuableVotes.json"
);
sol!(
    #[sol(rpc)]
    SimpleGovernance,
    "abi/damn-vulnerable-defi/SimpleGovernance.json"
);
sol!(
    #[sol(rpc)]
    SelfiePool,
    "abi/damn-vulnerable-defi/SelfiePool.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Selfie"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.damn_vulnerable_defi.selfie.address;
        Ok(Target { address })
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

        Ok(Target { address: *pool.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let pool = SelfiePool::new(self.address, provider.clone());

        println!("Checking that the pool is drained...");
        let token_addr = pool.token().call().await?;
        let token = DamnValuableVotes::new(token_addr, provider.clone());
        let pool_balance = token.balanceOf(self.address).call().await?;
        let offender_balance =
            token.balanceOf(roles.offender.address()).call().await?;

        Ok(pool_balance == U256::ZERO
            && offender_balance >= to_ether(1_500_000))
    }
}
