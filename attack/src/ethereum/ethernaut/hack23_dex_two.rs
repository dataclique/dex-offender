//! Ethernaut 23 — Dex Two.
//!
//! Goal: Drain both designated tokens from `DexTwo`. The DEX starts with 100 of
//! each token, and your offender account starts with ten of each.
//!
//
// Upstream hint:
//
// This level will ask you to break `DexTwo`, a subtly modified `Dex`
// contract from the previous level, in a different way.
//
// You need to drain all balances of token1 and token2 from the `DexTwo`
// contract to succeed in this level.
//
// You will still start with 10 tokens of `token1` and 10 of `token2`. The
// DEX contract still starts with 100 of each token.
//
// Things that might help:
// * How has the `swap` method been modified?
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/DexTwo.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl23_dex_two::*;

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
