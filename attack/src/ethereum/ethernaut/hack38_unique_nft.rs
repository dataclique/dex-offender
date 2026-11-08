//! Ethernaut 38 — Unique NFT.
//!
//! Goal: Own more than one NFT from `UniqueNFT` in your offender account.
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
