//! Ethernaut level 15 — NaughtCoin.
//!
//! Bypass the transfer lockup period of NaughtCoin.

use crate::{Level, Roles};
use alloy::{primitives::Address, primitives::U256, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    NaughtCoin,
    "abi/ethernaut/NaughtCoin.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "NaughtCoin"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethernaut.level15.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let offender = &roles.offender;
        let provider = roles.provider();

        println!("Deploying the NaughtCoin contract...");
        let contract = NaughtCoin::deploy(provider, offender.address()).await?;

        let _ = deployer;
        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = NaughtCoin::new(self.address, provider.clone());

        println!("Checking that you transferred all tokens...");
        let balance =
            contract.balanceOf(roles.offender.address()).call().await?;
        Ok(balance == U256::ZERO)
    }
}
