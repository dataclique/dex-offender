//! Ethernaut 26 — DoubleEntryPoint.
//!
//! Goal: Register a Forta detection bot for your offender account that blocks a
//! legacy-token sweep from `CryptoVault` while preserving a positive vault
//! balance of `DoubleEntryPoint` tokens.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/DoubleEntryPoint.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl26_double_entry_point::Target;

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
