//! Damn Vulnerable DeFi — The Rewarder.
//!
//! Claim all rewards from the distributor using crafted merkle proofs.

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, sol};
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
    WETH,
    "abi/damn-vulnerable-defi/WETH.json"
);
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    TheRewarderDistributor,
    "abi/damn-vulnerable-defi/TheRewarderDistributor.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "The Rewarder"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.damn_vulnerable_defi.the_rewarder.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        println!("Deploying the The Rewarder challenge...");
        let dvt = DamnValuableToken::deploy(provider).await?;
        let weth = WETH::deploy(provider).await?;
        let distributor = TheRewarderDistributor::deploy(provider).await?;

        weth.deposit()
            .value(to_ether(45))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        // TODO: compute merkle roots from distribution JSON data and call
        // distributor.createDistribution for both DVT and WETH
        let _ = dvt;

        Ok(Target { address: *distributor.address() })
    }

    async fn check(&self, _roles: &Roles) -> eyre::Result<bool> {
        // TODO: check DVT and WETH balances in distributor are near zero
        Ok(false)
    }
}
