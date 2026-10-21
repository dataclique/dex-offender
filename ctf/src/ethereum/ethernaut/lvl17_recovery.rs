//! Ethernaut level 17 — Recovery.
//!
//! Recover the address of a contract created by the Recovery factory.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Recovery,
    "abi/ethernaut/Recovery.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Recovery"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level17.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        println!("Deploying the Recovery contract...");
        let contract = Recovery::deploy(provider).await?;

        contract
            .generateToken(
                String::from("InitialToken"),
                alloy::primitives::U256::from(100000),
            )
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        // TODO: compute the token address from CREATE and fund it
        // The old ethers implementation used a helper contract; inline the
        // computation or add the helper ABI.

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();

        println!(
            "Checking that you found the token and took all ether from it..."
        );
        // TODO: compute the token address and check its balance is zero
        let _ = provider;
        Ok(false)
    }
}
