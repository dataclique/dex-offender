//! Ethernaut 08 — Vault.
//!
//! Goal: Unlock the password-protected `Vault` so that its `locked` flag
//! becomes false.
//!
//! Source: [Vault.sol][source]. [Rust harness][harness].
//! Test: `cargo test -p attack vault -- --ignored --nocapture`
//!
//! [source]:
//! ../../../../lib/ethernaut/contracts/src/levels/Vault.sol
//! [harness]: ctf::ethereum::ethernaut::lvl08_vault

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl08_vault::Target;

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
