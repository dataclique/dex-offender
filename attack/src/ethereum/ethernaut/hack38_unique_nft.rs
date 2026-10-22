//! Solution scaffold for Ethernaut level 38 — UniqueNFT.

// TODO: implement the exploit, then declare this module in
// `attack/src/ethernaut/mod.rs` to run its test.

use async_trait::async_trait;
use ctf::ethereum::ethernaut::lvl38_unique_nft::*;
use ethers::prelude::*;

pub(crate) struct Exploit;

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
