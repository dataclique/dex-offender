//! Ethernaut 40 — NotOptimisticPortal.
//!
//! Goal: Mint tokens through `NotOptimisticPortal`, a cross-chain portal
//! initialized with a synthetic genesis header. Its token total supply must
//! become positive.
//!
//
// Upstream hint:
//
// This portal relies on a complex chain of cryptographic proofs to verify
// cross-chain messages. It claims to be secure against invalid state
// transitions, but the gap between verification and execution might be wider
// than it looks.
//
// Can you manage to mint some tokens for your wallet?
//
// Things that might help:
//
// - Understanding Function Selectors.
//
// - The Checks-Effects-Interactions (CEI) pattern.
//
// - Merkle Patricia Tries and RLP encoding.
//
// Tips:
//
// - Sometimes the data you verify isn't exactly the same data you execute.
//
// - If a hash cycle seems impossible to solve, look for a way to break the
// loop.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/NotOptimisticPortal.sol

use async_trait::async_trait;
use ctf::ethereum::ethernaut::lvl40_not_optimistic_portal::*;

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
