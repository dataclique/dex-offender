//! Ethernaut 10 — Re-entrancy.
//!
//! Goal: Empty the Ether held by `Reentrance`, a contract that records
//! donations and lets users withdraw their balances.
//!
//! Source: [Reentrance.sol][source]. [Rust harness][harness].
//! Test: `cargo test -p attack reentrancy -- --ignored --nocapture`
//!
//! [source]:
//! ../../../../lib/ethernaut/contracts/src/levels/Reentrance.sol
//! [harness]: ctf::ethereum::ethernaut::lvl10_reentrancy

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl10_reentrancy::Target;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        // TODO: exploit the level
        let _ = (target, offender);
        Ok(())
    }
}
