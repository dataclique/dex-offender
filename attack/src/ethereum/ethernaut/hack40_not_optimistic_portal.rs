//! Ethernaut 40 — NotOptimisticPortal.
//!
//! Goal: Mint tokens through `NotOptimisticPortal`, a cross-chain portal
//! initialized with a synthetic genesis header. Its token total supply must
//! become positive.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/NotOptimisticPortal.sol

use async_trait::async_trait;
use ctf::ethereum::ethernaut::lvl40_not_optimistic_portal::*;

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
