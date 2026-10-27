//! Ethernaut 39 — Forger.
//!
//! Goal: Make the total supply of the `Forger` token exceed 100 whole tokens.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Forger.sol

use async_trait::async_trait;
use ctf::ethereum::ethernaut::lvl39_forger::*;

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
