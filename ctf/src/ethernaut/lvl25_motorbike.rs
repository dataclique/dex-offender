//! Ethernaut level 25 — Motorbike.
//!
//! Destroy the Motorbike engine by selfdestructing its implementation.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Motorbike,
    "abi/ethernaut/Motorbike.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Motorbike"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = Address::ZERO; // TODO: wire into Ethernaut struct
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        // TODO: implement deployment and initial state
        let _ = roles;
        unimplemented!("set_up not yet implemented")
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        // TODO: implement completion check
        let _ = roles;
        unimplemented!("check not yet implemented")
    }
}
