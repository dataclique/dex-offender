//! Ethernaut 25 — Motorbike.
//!
//! Goal: Leave the original `Engine` implementation behind the `Motorbike`
//! proxy without contract code.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Motorbike.sol

use crate::{Level, Roles};
use alloy::{
    primitives::{Address, B256, U256},
    providers::Provider,
    sol,
};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    Motorbike,
    "abi/ethernaut/Motorbike.json"
);

sol!(
    #[sol(rpc)]
    Engine,
    "abi/ethernaut/Engine.json"
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
        let address = ctfs.ethereum.ethernaut.level25.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the Motorbike contract...");
        // Mirror MotorbikeFactory: deploy the Engine implementation first,
        // then the proxy (whose constructor delegatecalls initialize()).
        let engine = Engine::deploy(provider).await?;
        let motorbike = Motorbike::deploy(provider, *engine.address()).await?;

        Ok(Target { address: *motorbike.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();

        println!("Checking that the engine is destroyed...");
        // Mirror MotorbikeFactory.validateInstance: the engine
        // implementation must no longer have code (selfdestructed).
        const EIP1967_IMPL_SLOT: B256 = alloy::primitives::b256!(
            "360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc"
        );
        let raw = provider
            .get_storage_at(
                self.address,
                U256::from_be_bytes(EIP1967_IMPL_SLOT.0),
            )
            .await?;
        let engine = Address::from_word(B256::from(raw.to_be_bytes()));
        let code = provider.get_code_at(engine).await?;
        Ok(code.is_empty())
    }
}
