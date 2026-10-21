//! Ethernaut level 26 — DoubleEntryPoint.
//!
//! Detect the DoubleEntryPoint vulnerability using a Forta bot.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    DoubleEntryPoint,
    "abi/ethernaut/DoubleEntryPoint.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "DoubleEntryPoint"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethernaut.level26.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let offender = &roles.offender;
        let provider = roles.provider();

        println!("Deploying the DoubleEntryPoint contract...");
        // The upstream factory deploys:
        // 1. LegacyToken (old token)
        // 2. CryptoVault (holds the tokens)
        // 3. Forta (detection bot registry)
        // 4. DoubleEntryPoint(newToken, vault, forta, player)
        // TODO: extract LegacyToken, CryptoVault, Forta ABIs
        let contract = DoubleEntryPoint::deploy(
            provider,
            Address::ZERO,
            Address::ZERO,
            Address::ZERO,
            offender.address(),
        )
        .await?;

        let _ = deployer;
        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = DoubleEntryPoint::new(self.address, provider.clone());

        println!("Checking that a bot raised an alert...");
        // Solved when the Forta bot has raised an alert about the vulnerability
        // TODO: check forta.users(player).botRaisedAlerts or similar
        let forta = contract.forta().call().await?;
        let _ = forta;
        Ok(false)
    }
}
