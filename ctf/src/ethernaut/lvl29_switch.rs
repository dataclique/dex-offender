//! Ethernaut level 29 — Switch.
//!
//! Turn on the Switch by manipulating its function selector.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Switch,
    "abi/ethernaut/Switch.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Switch"
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
