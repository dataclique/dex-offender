//! Ethernaut 37 — Impersonator Two.
//!
//! Goal: Empty the Ether held by the locked `ImpersonatorTwo` contract, which
//! starts with 0.001 ETH.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/ImpersonatorTwo.sol

use async_trait::async_trait;
use ctf::ethereum::ethernaut::lvl37_impersonator_two::*;

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
