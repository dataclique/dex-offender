//! Ethernaut 26 — DoubleEntryPoint.
//!
//! Goal: Register a Forta detection bot for your offender account that blocks a
//! legacy-token sweep from `CryptoVault` while preserving a positive vault
//! balance of `DoubleEntryPoint` tokens.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/DoubleEntryPoint.sol

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, primitives::U256, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    DoubleEntryPoint,
    "abi/ethernaut/DoubleEntryPoint.json"
);

sol!(
    #[sol(rpc)]
    LegacyToken,
    "abi/ethernaut/LegacyToken.json"
);

sol!(
    #[sol(rpc)]
    CryptoVault,
    "abi/ethernaut/CryptoVault.json"
);

sol!(
    #[sol(rpc)]
    Forta,
    "abi/ethernaut/Forta.json"
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
        let address = ctfs.ethereum.ethernaut.level26.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let offender = &roles.offender;
        let provider = roles.provider();

        // Mirror DoubleEntryPointFactory.createInstance.
        let old_token = LegacyToken::deploy(provider).await?;
        let forta = Forta::deploy(provider).await?;
        let vault = CryptoVault::deploy(provider, offender.address()).await?;
        let new_token = DoubleEntryPoint::deploy(
            provider,
            *old_token.address(),
            *vault.address(),
            *forta.address(),
            offender.address(),
        )
        .await?;

        vault
            .setUnderlying(*new_token.address())
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        old_token
            .delegateToNewContract(*new_token.address())
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        old_token
            .mint(*vault.address(), to_ether(100))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        println!("Deployed the DoubleEntryPoint at {}", new_token.address());
        Ok(Target { address: *new_token.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = DoubleEntryPoint::new(self.address, provider.clone());

        // Mirror validateInstance: solved when the player set a Forta
        // detection bot and sweeping the legacy token still reverts
        // (raised alert) while the vault keeps its balance.
        let forta_addr = contract.forta().call().await?;
        let forta = Forta::new(forta_addr, provider.clone());
        let bot =
            forta.usersDetectionBots(roles.offender.address()).call().await?;
        if bot == Address::ZERO {
            return Ok(false);
        }

        // Try sweeping the legacy (delegated-from) token: the alert raised by
        // the player's bot must make the sweep revert.
        let legacy = contract.delegatedFrom().call().await?;
        let vault_addr = contract.cryptoVault().call().await?;
        let vault = CryptoVault::new(vault_addr, provider.clone());
        let sweep = vault
            .sweepToken(legacy)
            .from(roles.offender.address())
            .send()
            .await;
        let reverted = sweep.is_err();
        let balance = contract.balanceOf(vault_addr).call().await?;
        Ok(reverted && balance > U256::ZERO)
    }
}
