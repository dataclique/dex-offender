//! Ethernaut 32 — Impersonator.
//!
//! Goal: Clear the controller of the first `ECLocker` created by
//! `Impersonator`, leaving `lockers(0).controller()` equal to the zero address.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Impersonator.sol

use async_trait::async_trait;
use ctf::ethereum::ethernaut::lvl32_impersonator::*;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        // TODO: exploit the level.
        let _ = (target, offender);
        Ok(())
    }
}
