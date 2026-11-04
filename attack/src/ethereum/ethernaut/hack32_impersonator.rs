//! Ethernaut 32 — Impersonator.
//!
//! Goal: Clear the controller of the first `ECLocker` created by
//! `Impersonator`, leaving `lockers(0).controller()` equal to the zero address.
//!
//
// Upstream hint:
//
// SlockDotIt’s new product, **ECLocker**, integrates IoT gate locks with
// Solidity smart contracts, utilizing Ethereum ECDSA for authorization. When
// a valid signature is sent to the lock, the system emits an `Open` event,
// unlocking doors for the authorized controller. SlockDotIt has hired you to
// assess the security of this product before its launch. Can you compromise
// the system in a way that anyone can open the door?
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Impersonator.sol

use async_trait::async_trait;
use ctf::ethereum::ethernaut::lvl32_impersonator::*;

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
