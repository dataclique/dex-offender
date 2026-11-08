//! Ethernaut 35 — Elliptic Token.
//!
//! Goal: Remove all of ALICE's `EllipticToken` tokens after her initial voucher
//! redemption. Her final token balance must be zero.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/EllipticToken.sol

use async_trait::async_trait;
use ctf::ethereum::ethernaut::lvl35_elliptic_token::*;

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
