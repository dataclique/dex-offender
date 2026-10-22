//! Damn Vulnerable DeFi — Backdoor.
//!
//! Drain tokens from wallets via a proxy creation callback backdoor.

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, primitives::U256, primitives::address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    DamnValuableToken,
    "abi/damn-vulnerable-defi/DamnValuableToken.json"
);
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Safe,
    "abi/damn-vulnerable-defi/Safe.json"
);
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    SafeProxyFactory,
    "abi/damn-vulnerable-defi/SafeProxyFactory.json"
);
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    WalletRegistry,
    "abi/damn-vulnerable-defi/WalletRegistry.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
    pub token: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Backdoor"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let dvd = &ctfs.ethereum.damn_vulnerable_defi.backdoor;
        Ok(Target { address: dvd.address, token: dvd.token })
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

        // Deploy registry with 4 beneficiaries
        let users = vec![
            address!("0000000000000000000000000000000000000001"), // placeholder — actual users are anvil accounts
            address!("0000000000000000000000000000000000000002"),
            address!("0000000000000000000000000000000000000003"),
            address!("0000000000000000000000000000000000000004"),
        ];
        let registry = WalletRegistry::deploy(
            provider,
            *singleton.address(),
            *factory.address(),
            *token.address(),
            users,
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

        Ok(Target { address: *registry.address(), token: *token.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let _registry = WalletRegistry::new(self.address, provider.clone());
        let token = DamnValuableToken::new(self.token, provider.clone());

        println!("Checking that the registry is drained...");
        let balance = token.balanceOf(self.address).call().await?;
        let offender_balance =
            token.balanceOf(roles.offender.address()).call().await?;

        Ok(balance == U256::ZERO && offender_balance >= to_ether(40))
    }
}
