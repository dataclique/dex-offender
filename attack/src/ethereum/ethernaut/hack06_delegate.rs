//! Ethernaut 06 — Delegation.
//!
//! Goal: Take ownership of the `Delegation` instance, which uses a companion
//! `Delegate` contract. The checked owner belongs to `Delegation`, not
//! `Delegate`.
//!
//! Source: [Delegation.sol][source]. [Rust harness][harness].
//! Test: `cargo test -p attack delegate -- --ignored --nocapture`
//!
//! [source]:
//! ../../../../lib/ethernaut/contracts/src/levels/Delegation.sol
//! [harness]: ctf::ethereum::ethernaut::lvl06_delegate

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl06_delegate::Target;

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
