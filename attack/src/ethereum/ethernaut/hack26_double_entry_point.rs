//! Ethernaut 26 — DoubleEntryPoint.
//!
//! Goal: Register a Forta detection bot for your offender account that blocks a
//! legacy-token sweep from `CryptoVault` while preserving a positive vault
//! balance of `DoubleEntryPoint` tokens.
//!
//
// Upstream hint:
//
// This level features a `CryptoVault` with special functionality, the
// `sweepToken` function. This is a common function used to retrieve tokens
// stuck in a contract. The `CryptoVault` operates with an `underlying` token
// that can't be swept, as it is an important core logic component of the
// `CryptoVault`. Any other tokens can be swept.
//
// The underlying token is an instance of the DET token implemented in the
// `DoubleEntryPoint` contract definition and the `CryptoVault` holds 100
// units of it. Additionally the `CryptoVault` also holds 100 of `LegacyToken
// LGT`.
//
// In this level you should figure out where the bug is in `CryptoVault` and
// protect it from being drained out of tokens.
//
// The contract features a `Forta` contract where any user can register its
// own `detection bot` contract. Forta is a decentralized, community-based
// monitoring network to detect threats and anomalies on DeFi, NFT,
// governance, bridges and other Web3 systems as quickly as possible. Your
// job is to implement a `detection bot` and register it in the `Forta`
// contract. The bot's implementation will need to raise correct alerts to
// prevent potential attacks or bug exploits.
//
// Things that might help:
// - How does a double entry point work for a token contract?
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/DoubleEntryPoint.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl26_double_entry_point::*;

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
