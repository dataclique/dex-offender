//! Ethernaut 24 — Puzzle Wallet.
//!
//! Goal: Become the `admin` of `PuzzleProxy`, the proxy that runs the
//! `PuzzleWallet` implementation.
//!
//
// Upstream hint:
//
// Nowadays, paying for DeFi operations is impossible, fact.
//
// A group of friends discovered how to slightly decrease the cost of
// performing multiple transactions by batching them in one transaction, so
// they developed a smart contract for doing this.
//
// They needed this contract to be upgradeable in case the code contained a
// bug, and they also wanted to prevent people from outside the group from
// using it. To do so, they voted and assigned two people with special roles
// in the system:
// The admin, which has the power of updating the logic of the smart
// contract.
// The owner, which controls the whitelist of addresses allowed to use the
// contract.
// The contracts were deployed, and the group was whitelisted. Everyone
// cheered for their accomplishments against evil miners.
//
// Little did they know, their lunch money was at risk…
//
// You'll need to hijack this wallet to become the admin of the proxy.
//
// Things that might help:
// * Understanding how `delegatecall` works and how `msg.sender` and
// `msg.value` behaves when performing one.
// * Knowing about proxy patterns and the way they handle storage variables.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/PuzzleWallet.sol
//!
//! The proxy's base implementation:
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/helpers/UpgradeableProxy-08.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl24_puzzle_wallet::*;

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
