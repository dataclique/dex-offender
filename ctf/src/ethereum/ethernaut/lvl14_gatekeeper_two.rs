//! Ethernaut 14 — Gatekeeper Two.
//!
//! Goal: Pass all three gates of `GatekeeperTwo` and register your offender
//! account as its `entrant`.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/GatekeeperTwo.sol

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    GatekeeperTwo,
    "abi/ethernaut/GatekeeperTwo.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "GatekeeperTwo"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level14.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the GatekeeperTwo contract...");
        let contract = GatekeeperTwo::deploy(provider).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = GatekeeperTwo::new(self.address, provider.clone());

        let entrant = contract.entrant().call().await?;
        Ok(entrant == roles.offender.address())
    }
}
