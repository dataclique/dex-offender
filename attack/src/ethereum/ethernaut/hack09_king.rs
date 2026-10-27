//! Ethernaut 09 — King.
//!
//! Goal: Claim the throne in `King` and prevent the original owner from
//! reclaiming it during the level check.
//!
//! Source: [King.sol][source]. [Rust harness][harness].
//! Test: `cargo test -p attack king -- --ignored --nocapture`
//!
//! [source]:
//! ../../../../lib/ethernaut/contracts/src/levels/King.sol
//! [harness]: ctf::ethereum::ethernaut::lvl09_king

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl09_king::Target;

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
