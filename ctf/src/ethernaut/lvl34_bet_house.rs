//! Ethernaut level 34 — Bet House.
//!
//! Master the art of strategic gambling and become a bettor.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    BetHouse,
    "abi/ethernaut/BetHouse.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Bet House"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethernaut.level34.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the BetHouse contract...");
        // TODO: deploy pool token first
        let contract = BetHouse::deploy(provider, Address::ZERO).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = BetHouse::new(self.address, provider.clone());

        println!("Checking that you are a bettor...");
        let is_bettor =
            contract.isBettor(roles.offender.address()).call().await?;
        Ok(is_bettor)
    }
}
