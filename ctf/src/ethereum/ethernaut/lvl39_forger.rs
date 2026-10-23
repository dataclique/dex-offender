//! Ethernaut level 39 — Forger.
//!
//! Make the total supply greater than 100 tokens.

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Forger,
    "abi/ethernaut/Forger.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Forger"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level39.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        // Mirror ForgerFactory: a bare Forger deployment. The golden
        // signature is fixed inside the contract; no extra setup needed.
        let contract = Forger::deploy(provider).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = Forger::new(self.address, provider.clone());

        println!("Checking that total supply > 100 ether...");
        let supply = contract.totalSupply().call().await?;
        Ok(supply > to_ether(100))
    }
}
