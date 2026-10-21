//! Ethernaut level 14 — GatekeeperTwo.
//!
//! Pass all three gates of GatekeeperTwo using extcodesize and type
//! truncation tricks.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
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

        println!("Checking the entrant...");
        let entrant = contract.entrant().call().await?;
        Ok(entrant == roles.offender.address())
    }
}
