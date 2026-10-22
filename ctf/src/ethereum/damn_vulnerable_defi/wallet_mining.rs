//! Damn Vulnerable DeFi — Wallet Mining.
//!
//! Deploy wallets without governance approval and drain the WalletDeployer.
//!
//! TODO: This is the most complex DVD setup — requires deploying Safe
//! Singleton Factory and CreateX via pre-signed raw transactions, then
//! AuthorizerUpgradeable with proxy, SafeProxyFactory, and WalletDeployer.

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
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
    WalletDeployer,
    "abi/damn-vulnerable-defi/WalletDeployer.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
    pub token: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Wallet Mining"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let dvd = ctfs
            .ethereum
            .damn_vulnerable_defi
            .wallet_mining
            .as_ref()
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?;
        Ok(Target { address: dvd.address, token: dvd.token })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        // TODO: full setup requires Safe Singleton Factory + CreateX via
        // pre-signed raw transactions. This is the most complex DVD level.
        let _ = roles;
        unimplemented!(
            "Wallet Mining setup not yet implemented — requires Safe factory + CreateX raw tx deploy"
        )
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let _token = DamnValuableToken::new(self.token, provider.clone());

        println!("Checking that all funds are stolen...");
        // Solved when all DVT tokens are recovered
        // TODO: check specific conditions from upstream test
        Ok(false)
    }
}
