//! Ethernaut 18 — Magic Number.
//!
//! Goal: Register a solver with `MagicNum` that returns the 32-byte
//! representation of 42 from `whatIsTheMeaningOfLife()`. Its deployed runtime
//! must be at most ten bytes.
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
