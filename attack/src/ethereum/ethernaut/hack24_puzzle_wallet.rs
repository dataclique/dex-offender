//! Ethernaut 24 — Puzzle Wallet.
//!
//! Goal: Become the `admin` of `PuzzleProxy`, the proxy that runs the
//! `PuzzleWallet` implementation.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/PuzzleWallet.sol
//!
//! The proxy's base implementation:
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/helpers/UpgradeableProxy-08.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl24_puzzle_wallet::Target;

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
