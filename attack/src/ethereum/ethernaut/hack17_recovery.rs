//! Ethernaut 17 — Recovery.
//!
//! Goal: Recover all Ether from the lost `SimpleToken` created by `Recovery`.
//! The token's Ether balance, not the factory's, must become zero.
//!
//
// Upstream hint:
//
// A contract creator has built a very simple token factory contract. Anyone
// can create new tokens with ease. After deploying the first token contract,
// the creator sent `0.001` ether to obtain more tokens. They have since lost
// the contract address.
//
// This level will be completed if you can recover (or remove) the `0.001`
// ether from the lost contract address.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Recovery.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl17_recovery::*;

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
