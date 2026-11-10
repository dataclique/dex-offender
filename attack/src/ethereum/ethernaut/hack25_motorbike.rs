//! Ethernaut 25 — Motorbike.
//!
//! Goal: Leave the original `Engine` implementation behind the `Motorbike`
//! proxy without contract code.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Motorbike.sol

use alloy::primitives::{Address, B256, U256, b256};
use alloy::providers::Provider;
use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl25_motorbike::*;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        crate::hint!();
        let provider = offender.provider();
        let motorbike = Motorbike::new(target.address, provider.clone());
        println!("Motorbike: {}", motorbike.address());

        const EIP1967_IMPL_SLOT: B256 = b256!(
            "0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc"
        );

        let raw = provider
            .get_storage_at(
                *motorbike.address(),
                U256::from_be_bytes(EIP1967_IMPL_SLOT.0),
            )
            .await?;

        let engine_addr = Address::from_word(B256::from(raw.to_be_bytes()));
        let engine = Engine::new(engine_addr, provider.clone());

        println!("Engine: {}", engine.address());

        Ok(())
    }
}
