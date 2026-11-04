//! Ethernaut 12 — Privacy.
//!
//! Goal: Unlock the `Privacy` contract so that its `locked` flag becomes false.
//!
//
// Upstream hint:
//
// The creator of this contract was careful enough to protect the sensitive
// areas of its storage.
//
// Unlock this contract to beat the level.
//
// Things that might help:
// * Understanding how storage works
// * Understanding how parameter parsing works
// * Understanding how casting works
//
// Tips:
// * Remember that metamask is just a commodity. Use another tool if it is
// presenting problems. Advanced gameplay could involve using remix, or your
// own web3 provider.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Privacy.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl12_privacy::*;

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
