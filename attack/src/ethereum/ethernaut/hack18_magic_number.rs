//! Ethernaut 18 — Magic Number.
//!
//! Goal: Register a solver with `MagicNum` that returns the 32-byte
//! representation of 42 from `whatIsTheMeaningOfLife()`. Its deployed runtime
//! must be at most ten bytes.
//!
//
// Upstream hint:
//
// To solve this level, you only need to provide the Ethernaut with a
// `Solver`, a contract that responds to `whatIsTheMeaningOfLife()` with the
// right 32 byte number.
//
// Easy right?
// Well... there's a catch.
//
// The solver's code needs to be really tiny. Really reaaaaaallly tiny. Like
// freakin' really really itty-bitty tiny: 10 bytes at most.
//
// Hint: Perhaps its time to leave the comfort of the Solidity compiler
// momentarily, and build this one by hand O_o.
// That's right: Raw EVM bytecode.
//
// Good luck!
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/MagicNum.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl18_magic_number::*;

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
