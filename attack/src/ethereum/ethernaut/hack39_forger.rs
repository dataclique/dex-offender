//! Ethernaut 39 — Forger.
//!
//! Goal: Make the total supply of the `Forger` token exceed 100 whole tokens.
//!
//
// Upstream hint:
//
// This is the Forger, the token printer your mom warned you about.
//
// This ERC-20 hands out mint passes signed by the owner... or so they say.
//
// One golden signature already exists, good for 100 shiny tokens.
//
// The team insists the pass is single-use and perfectly safe.
//
// Your goal? Making the total supply greater than 100 tokens.
//
// Get creative, stay sharp, and may your forgeries be legendary.
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
