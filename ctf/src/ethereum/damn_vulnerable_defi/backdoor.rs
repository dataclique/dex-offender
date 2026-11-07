//! Damn Vulnerable DeFi — Backdoor.
//!
//! Drain tokens from wallets via a proxy creation callback backdoor.

use super::recovery_account;
use crate::{Level, Roles, to_ether};
use alloy::{
    primitives::Address, primitives::address, providers::Provider, sol,
};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    DamnValuableToken,
    "../lib/damn-vulnerable-defi/out/DamnValuableToken.sol/DamnValuableToken.json"
);
sol!(
    #[sol(rpc)]
    Safe,
    "../lib/damn-vulnerable-defi/out/Safe.sol/Safe.json"
);
sol!(
    #[sol(rpc)]
    SafeProxyFactory,
    "../lib/damn-vulnerable-defi/out/SafeProxyFactory.sol/SafeProxyFactory.json"
);
sol!(
    #[sol(rpc)]
    WalletRegistry,
    "../lib/damn-vulnerable-defi/out/WalletRegistry.sol/WalletRegistry.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
    pub token: Address,
    /// The four registered beneficiaries (upstream: alice, bob, charlie,
    /// david).
    pub users: Vec<Address>,
    /// The designated recovery account.
    pub recovery: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Backdoor"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethereum.damn_vulnerable_defi.backdoor.clone())
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        println!("Deploying the Backdoor challenge...");

        // Deploy Safe singleton and factory
        let singleton = Safe::deploy(provider).await?;
        let factory = SafeProxyFactory::deploy(provider).await?;

        // Deploy token
        let token = DamnValuableToken::deploy(provider).await?;

        // Deploy registry with the four upstream beneficiaries (anvil
        // accounts 5-8: alice, bob, charlie, david).
        let users = vec![
            address!("9965507D1a55bcC2695C58ba16FB37d819B0A4dc"),
            address!("976EA74026E726554dB657fA54763abd0C3a0aa9"),
            address!("14dC79964da2C08b23698B3D3cc7Ca32193d9955"),
            address!("23618e81E3f5cdF7f54C3d65f7FBc0aBf5B21E8f"),
        ];
        let registry = WalletRegistry::deploy(
            provider,
            *singleton.address(),
            *factory.address(),
            *token.address(),
            users.clone(),
        )
        .await?;

        // Fund registry
        token
            .transfer(*registry.address(), to_ether(40))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        Ok(Target {
            address: *registry.address(),
            token: *token.address(),
            users,
            recovery: recovery_account(),
        })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let offender = &roles.offender;
        let provider = roles.provider();
        let registry = WalletRegistry::new(self.address, provider.clone());
        let token = DamnValuableToken::new(self.token, provider.clone());

        println!(
            "Checking that all wallets were drained to the recovery account..."
        );
        // Upstream success: one player transaction, every beneficiary has a
        // registered wallet and lost beneficiary status, and the recovery
        // account owns the entire distributed amount.
        let mut wallets_registered = true;
        let mut beneficiaries_removed = true;
        for user in &self.users {
            wallets_registered &=
                registry.wallets(*user).call().await? != Address::ZERO;
            beneficiaries_removed &=
                !registry.beneficiaries(*user).call().await?;
        }
        Ok(provider.get_transaction_count(offender.address()).await? == 1
            && wallets_registered
            && beneficiaries_removed
            && token.balanceOf(self.recovery).call().await? == to_ether(40))
    }
}
