//! Ethernaut level 24 — PuzzleWallet.
//!
//! Become the owner and drain the PuzzleWallet proxy.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    PuzzleWallet,
    "abi/ethernaut/PuzzleWallet.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "PuzzleWallet"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level24.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        println!("Deploying the PuzzleWallet contract...");
        // The upstream factory:
        // 1. deploys PuzzleWallet logic
        // 2. encodes init(100 ether) as proxy data
        // 3. deploys PuzzleProxy(admin, logic, data)
        // 4. whitelists the factory and deposits 0.001 ETH
        // TODO: extract PuzzleProxy ABI and implement the proxy deployment
        let contract = PuzzleWallet::deploy(provider).await?;

        let _ = deployer;
        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = PuzzleWallet::new(self.address, provider.clone());

        println!("Checking that you became the owner...");
        let owner = contract.owner().call().await?;
        Ok(owner == roles.offender.address())
    }
}
