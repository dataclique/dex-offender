//! Ethernaut 14 — Gatekeeper Two.
//!
//! Goal: Pass all three gates of `GatekeeperTwo` and register your offender
//! account as its `entrant`.
//!
//
// Upstream hint:
//
// This gatekeeper introduces a few new challenges. Register as an entrant to
// pass this level.
//
// ##### Things that might help:
// * Remember what you've learned from getting past the first gatekeeper -
// the first gate is the same.
// * The `assembly` keyword in the second gate allows a contract to access
// functionality that is not native to vanilla Solidity. See [Solidity
// Assembly](http://solidity.readthedocs.io/en/v0.4.23/assembly.html) for
// more information. The `extcodesize` call in this gate will get the size of
// a contract's code at a given address - you can learn more about how and
// when this is set in section 7 of the [yellow
// paper](https://ethereum.github.io/yellowpaper/paper.pdf).
// * The `^` character in the third gate is a bitwise operation (XOR), and is
// used here to apply another common bitwise operation (see [Solidity
// cheatsheet](http://solidity.readthedocs.io/en/v0.4.23/miscellaneous.html#cheatsheet)).
// The Coin Flip level is also a good place to start when approaching this
// challenge.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/GatekeeperTwo.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl14_gatekeeper_two::*;

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
