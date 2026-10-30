//! Paradigm CTF 2023 — Hello World.
//!
//! Solve the challenge by calling `solve()` on the challenge contract.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    Challenge,
    "../lib/paradigm-ctf-2023/hello-world/challenge/project/out/Challenge.sol/Challenge.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Hello World"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.paradigm_2023.hello_world.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the Hello World challenge...");
        let contract = Challenge::deploy(provider).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = Challenge::new(self.address, provider.clone());

        let solved = contract.isSolved().call().await?;
        Ok(solved)
    }
}
