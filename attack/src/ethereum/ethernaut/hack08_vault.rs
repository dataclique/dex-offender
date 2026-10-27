//! Ethernaut level 08 — Vault.
//!
//! Goal: Unlock the password-protected `Vault` so that its `locked` flag becomes false.
//!
//! Source: [local](../../../../lib/ethernaut/contracts/src/levels/Vault.sol) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/contracts/src/levels/Vault.sol).
//! Challenge: [local](../../../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/vault.md) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/client/src/gamedata/en/descriptions/levels/vault.md).
//! [Factory success check](../../../../lib/ethernaut/contracts/src/levels/VaultFactory.sol) · [Rust bindings](ctf::ethereum::ethernaut::lvl08_vault).
//!
//! `target` contains the deployed instance address and any level-specific helpers.
//! `offender` is your player account; sign transactions with `offender.address()`.

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
