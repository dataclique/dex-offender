//! Ethernaut 40 — NotOptimisticPortal.
//!
//! Goal: Mint tokens through `NotOptimisticPortal`, a cross-chain portal
//! initialized with a synthetic genesis header. Its token total supply must
//! become positive.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/NotOptimisticPortal.sol

use crate::{Level, Roles};
use alloy::{primitives::Address, primitives::U256, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    NotOptimisticPortal,
    "abi/ethernaut/NotOptimisticPortal.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "NotOptimisticPortal"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level40.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        // Mirror NotOptimisticPortalFactory: deploy the portal with the
        // same synthetic genesis block header and a burn governance address.
        let genesis_rlp = alloy::primitives::hex::decode(
            "f90204a00000000000000000000000000000000000000000000000000000000000000000a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347940000000000000000000000000000000000000000a0d7d3685b57d9897755fad850b19f7c43debfded002e18a9e8e5b63639882b6f9a0c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470a0c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000184039fd3988401c9c38080845fc630578b4354465f5061796c6f6164a00000000000000000000000000000000000000000000000000000000000000000880000000000000000",
        )?;
        let governance = Address::from_word(alloy::primitives::keccak256(
            "governance".as_bytes(),
        ));
        let contract = NotOptimisticPortal::deploy(
            provider,
            "CTFToken".to_string(),
            "CTFT".to_string(),
            genesis_rlp.into(),
            governance,
        )
        .await?;

        println!("Deployed the NotOptimisticPortal at {}", contract.address());
        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = NotOptimisticPortal::new(self.address, provider.clone());

        println!("Checking that tokens were minted...");
        // Mirror validateInstance: total supply must exceed zero.
        let supply = contract.totalSupply().call().await?;
        Ok(supply > U256::ZERO)
    }
}
