//! Ethernaut 38 — Unique NFT.
//!
//! Goal: Own more than one NFT from `UniqueNFT` in your offender account.
//!
//
// Upstream hint:
//
// Welcome to UniqueNFT, where any user can get its very own shiny digital
// badge.
//
// Humans with EOAs? You mint for free, no questions asked. Proof of Humanity
// for the win!
//
// Smart contracts? Sorry bots, pay the toll –> one whole ether!
//
// But here’s the twist: one badge per address, no greedy hoarding allowed.
//
// And forget about trading, these things stick like glue.
//
// It’s like blockchain tattoos, once it’s yours, it’s yours forever.
//
// Think you can outsmart the rules and own more a single NFT? Prove it.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/UniqueNFT.sol

use async_trait::async_trait;
use ctf::ethereum::ethernaut::lvl38_unique_nft::*;

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
