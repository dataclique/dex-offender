//! Ethernaut 05 — Token.
//!
//! Goal: Increase your offender account's `Token` balance above the twenty
//! tokens it receives at setup.
//!
//! Source: [Token.sol][source]. [Rust harness][harness].
//! Test: `cargo test -p attack token -- --ignored --nocapture`
//!
//! [source]:
//! ../../../../lib/ethernaut/contracts/src/levels/Token.sol
//! [harness]: ctf::ethereum::ethernaut::lvl05_token

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl05_token::Target;

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
