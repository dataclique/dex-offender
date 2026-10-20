//! Solution scaffold for Damn Vulnerable DeFi level 16 — Shards.

// TODO: implement the exploit, then declare this module in
// `attack/src/damn_vulnerable_defi/mod.rs` to run its test.

use async_trait::async_trait;
use ctf::damn_vulnerable_defi::lvl16_shards::*;
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
