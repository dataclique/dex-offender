//! Damn Vulnerable DeFi — Unstoppable.
//!
//! Make the vault stop offering flash loans.

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, primitives::U256, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UnstoppableVault,
    "abi/damn-vulnerable-defi/UnstoppableVault.json"
);
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    DamnValuableToken,
    "abi/damn-vulnerable-defi/DamnValuableToken.json"
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

        println!("Checking that the vault has stopped offering flash loans...");
        // Completed when the vault can no longer service flash loans
        // (its internal accounting invariant is broken)
        let asset = vault.asset().call().await?;
        let total_supply = vault.totalSupply().call().await?;
        let asset_balance = DamnValuableToken::new(asset, provider.clone())
            .balanceOf(self.address)
            .call()
            .await?;

        Ok(asset_balance > total_supply && total_supply > U256::ZERO)
    }
}
