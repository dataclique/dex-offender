//! Ethernaut 36 — Cashback.
//!
//! Goal: Reach the maximum native-currency and FREE cashback balances in
//! `Cashback`, own at least two Super Cashback NFTs including your
//! address-derived NFT, and leave your account delegated to the Cashback
//! instance as required by its factory check.
//!
//
// Upstream hint:
//
// You’ve just joined Cashback, the hottest crypto neobank in town. Their
// pitch is irresistible: for every on-chain payment you make, you earn
// points. Rack up enough and you’ll reach legendary status, unlocking the
// coveted Super Cashback NFT badge.
//
// The system leverages EIP-7702 to allow EOAs to accrue cashback. Users must
// delegate to the Cashback contract to use the `payWithCashback` function.
//
// Rumor has it there’s a back door for power users. Your brief is simple:
// become the loyalty program’s nightmare. Max out your cashback in every
// supported currency and walk away with at least two Super Cashback NFT, one
// of which must correspond to your player address.
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
