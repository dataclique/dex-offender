//! Damn Vulnerable DeFi — Unstoppable.
//!
//! Make the vault stop offering flash loans.

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    UnstoppableVault,
    "../lib/damn-vulnerable-defi/out/UnstoppableVault.sol/UnstoppableVault.json"
);
sol!(
    #[sol(rpc)]
    DamnValuableToken,
    "../lib/damn-vulnerable-defi/out/DamnValuableToken.sol/DamnValuableToken.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Unstoppable"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.damn_vulnerable_defi.unstoppable.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let offender = &roles.offender;
        let provider = roles.provider();

        println!("Deploying the Unstoppable challenge...");

        let tokens_in_vault = to_ether(1_000_000);
        let initial_offender_balance = to_ether(10);

        let token = DamnValuableToken::deploy(provider).await?;
        let vault = UnstoppableVault::deploy(
            provider,
            *token.address(),
            deployer.address(),
            deployer.address(),
        )
        .await?;

        token
            .approve(*vault.address(), tokens_in_vault)
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        vault
            .deposit(tokens_in_vault, deployer.address())
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        token
            .transfer(offender.address(), initial_offender_balance)
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        Ok(Target { address: *vault.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let vault = UnstoppableVault::new(self.address, provider.clone());

        // Upstream: the monitor pauses the vault and returns ownership to
        // the deployer once the invariant breaks.
        let paused = vault.paused().call().await?;
        let owner = vault.owner().call().await?;
        Ok(paused && owner == roles.deployer.address())
    }
}
