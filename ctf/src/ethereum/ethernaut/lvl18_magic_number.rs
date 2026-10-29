//! Ethernaut 18 — Magic Number.
//!
//! Goal: Register a solver with `MagicNum` that returns the 32-byte
//! representation of 42 from `whatIsTheMeaningOfLife()`. Its deployed runtime
//! must be at most ten bytes.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/MagicNum.sol

use crate::{Level, Roles};
use alloy::{primitives::Address, primitives::U256, providers::Provider, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    MagicNum,
    "abi/ethernaut/MagicNum.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "MagicNum"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level18.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the MagicNum contract...");
        let contract = MagicNum::deploy(provider).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = MagicNum::new(self.address, provider.clone());

        let solver_address = contract.solver().call().await?;
        if solver_address == Address::ZERO {
            return Ok(false);
        }

        let code = provider.get_code_at(solver_address).await?;
        if code.len() > 10 {
            println!("Contract too large: {} bytes", code.len());
            return Ok(false);
        }

        // Call whatIsTheMeaningOfLife() on the solver
        // The selector is keccak256("whatIsTheMeaningOfLife()")[0..4]
        let selector =
            alloy::primitives::keccak256("whatIsTheMeaningOfLife()".as_bytes());
        let calldata = alloy::primitives::Bytes::from(selector[..4].to_vec());
        let result = provider
            .call(
                alloy::rpc::types::TransactionRequest::default()
                    .to(solver_address)
                    .input(calldata.into()),
            )
            .await;

        match result {
            Ok(data) => {
                let value = U256::try_from_be_slice(
                    &data[..core::cmp::min(32, data.len())],
                )
                .unwrap_or_default();
                Ok(value == U256::from(42))
            }
            Err(_) => Ok(false),
        }
    }
}
