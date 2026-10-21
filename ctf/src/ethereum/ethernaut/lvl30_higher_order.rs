//! Ethernaut level 30 — HigherOrder.
//!
//! Become the Commander of the Higher Order.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    HigherOrder,
    "abi/ethernaut/HigherOrder.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "HigherOrder"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level30.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the HigherOrder contract...");
        let contract = HigherOrder::deploy(provider).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = HigherOrder::new(self.address, provider.clone());

        println!("Checking that you are the Commander...");
        let commander = contract.commander().call().await?;
        Ok(commander == roles.offender.address())
    }
}
