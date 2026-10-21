//! Template for new Ethernaut level harness modules.
//!
//! Copy this file, rename it `lvlNN_name.rs`, replace `REPLACE_ME` with the
//! contract name, and fill in the set_up and check logic. Then declare the
//! module in `ctf/src/ethernaut/mod.rs` and add the field to `Ethernaut`.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    REPLACE_ME,
    "abi/ethernaut/REPLACE_ME.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "ReplaceMe"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.levelNN.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the ReplaceMe contract...");
        let contract = REPLACE_ME::deploy(provider).await?;

        // TODO: deploy helper contracts, initialize state, fund accounts

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = REPLACE_ME::new(self.address, provider.clone());

        // TODO: check the completion condition

        Ok(false)
    }
}
