//! Ethernaut 16 — Preservation.
//!
//! Goal: Take ownership of `Preservation`, a timezone-recording contract that
//! uses two `LibraryContract` instances.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Preservation.sol

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    Preservation,
    "../lib/ethernaut/contracts/out/Preservation.sol/Preservation.json"
);
sol!(
    #[sol(rpc)]
    LibraryContract,
    "../lib/ethernaut/contracts/out/Preservation.sol/LibraryContract.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Preservation"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level16.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the Preservation contract...");
        // Mirror PreservationFactory: two real LibraryContract instances
        // back the timezone slots.
        let time_zone1_library = LibraryContract::deploy(provider).await?;
        let time_zone2_library = LibraryContract::deploy(provider).await?;
        let contract = Preservation::deploy(
            provider,
            *time_zone1_library.address(),
            *time_zone2_library.address(),
        )
        .await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = Preservation::new(self.address, provider.clone());

        let owner = contract.owner().call().await?;
        Ok(owner == roles.offender.address())
    }
}
