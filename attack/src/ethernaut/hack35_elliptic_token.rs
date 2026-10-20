//! Solution scaffold for Ethernaut level 35 — Elliptic Token.

// TODO: implement the exploit, then declare this module in
// `attack/src/ethernaut/mod.rs` to run its test.

use async_trait::async_trait;
use ctf::ethernaut::lvl35_elliptic_token::*;
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
