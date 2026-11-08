//! Ethernaut 34 — Bet House.
//!
//! Goal: Become a bettor in `BetHouse`, which uses a `Pool` and two `PoolToken`
//! contracts. Your offender account receives five deposit tokens at setup;
//! `isBettor(offender)` must become true.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/BetHouse.sol

use async_trait::async_trait;
use ctf::ethereum::ethernaut::lvl34_bet_house::*;

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
