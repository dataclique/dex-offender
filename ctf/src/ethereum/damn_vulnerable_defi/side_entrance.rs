//! Damn Vulnerable DeFi — Side Entrance.
//!
//! Use the pool's deposit/withdraw to drain all ETH.

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, primitives::U256, providers::Provider, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    SideEntranceLenderPool,
    "abi/damn-vulnerable-defi/SideEntranceLenderPool.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Side Entrance"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.damn_vulnerable_defi.side_entrance.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        println!("Deploying the Side Entrance challenge...");
        let pool = SideEntranceLenderPool::deploy(provider).await?;

        // Fund the pool via deposit
        pool.deposit()
            .value(to_ether(1000))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        Ok(Target { address: *pool.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();

        println!("Checking that the pool is drained...");
        let balance = provider.get_balance(self.address).await?;
        Ok(balance == U256::ZERO)
    }
}
