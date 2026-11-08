//! Ethernaut 18 — Magic Number.
//!
//! Goal: Register a solver with `MagicNum` that returns the 32-byte
//! representation of 42 from `whatIsTheMeaningOfLife()`. Its deployed runtime
//! must be at most ten bytes.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/MagicNum.sol

use std::path::PathBuf;
use std::str::FromStr;

use alloy::primitives::Bytes;
use alloy::providers::Provider;
use alloy::rpc::types::{TransactionInput, TransactionRequest};
use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl18_magic_number::*;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        let cwd = std::env::current_dir()?;
        let compiled_path = PathBuf::from_str(
            "attack/out/SolverExploit.yul/SolverExploit.json",
        )?;

        let output = std::fs::read_to_string(cwd.join(compiled_path))?;
        let json: serde_json::Value = serde_json::from_str(&output)?;
        let bytecode = &json["bytecode"]["object"].as_str().unwrap();
        let input = Bytes::from_str(bytecode)?;

        let provider = offender.provider();
        let tx = TransactionRequest::default()
            .create()
            .input(TransactionInput::both(input));

        let hash = provider.send_transaction(tx).await?.watch().await?;
        let receipt = provider.get_transaction_receipt(hash).await?;
        let solver = receipt.unwrap().contract_address.unwrap();

        let contract = MagicNum::new(target.address, provider.clone());
        contract.setSolver(solver).send().await?.watch().await?;

        Ok(())
    }
}
