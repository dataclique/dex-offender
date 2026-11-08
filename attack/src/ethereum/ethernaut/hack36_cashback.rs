//! Ethernaut 36 — Cashback.
//!
//! Goal: Reach the maximum native-currency and FREE cashback balances in
//! `Cashback`, own at least two Super Cashback NFTs including your
//! address-derived NFT, and leave your account delegated to the Cashback
//! instance as required by its factory check.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Cashback.sol
//!
//! SuperCashbackNFT, FreedomCoin and their setup:
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/CashbackFactory.sol

use async_trait::async_trait;
use ctf::ethereum::ethernaut::lvl36_cashback::*;

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
