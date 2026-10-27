//! Ethernaut 04 — Telephone.
//!
//! Goal: Make your offender account the owner of the `Telephone` contract.
//!
//! Source: [Telephone.sol][source]. [Rust harness][harness].
//! Test: `cargo test -p attack telephone -- --ignored --nocapture`
//!
//! [source]:
//! ../../../../lib/ethernaut/contracts/src/levels/Telephone.sol
//! [harness]: ctf::ethereum::ethernaut::lvl04_telephone

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl04_telephone::Target;

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
