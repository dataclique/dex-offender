//! Paradigm CTF 2023 — Dai Plus Plus.
//!
//! Exploit the stablecoin system.

use crate::{Level, Roles, anvil::plant_eth_usd_feed};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    SystemConfiguration,
    "../lib/paradigm-ctf-2023/dai-plus-plus/challenge/project/out/SystemConfiguration.sol/SystemConfiguration.json"
);

sol!(
    #[sol(rpc)]
    AccountManager,
    "../lib/paradigm-ctf-2023/dai-plus-plus/challenge/project/out/AccountManager.sol/AccountManager.json"
);

sol!(
    #[sol(rpc)]
    Stablecoin,
    "../lib/paradigm-ctf-2023/dai-plus-plus/challenge/project/out/Stablecoin.sol/Stablecoin.json"
);

sol!(
    #[sol(rpc)]
    AccountImpl,
    "../lib/paradigm-ctf-2023/dai-plus-plus/challenge/project/out/Account.sol/Account.json"
);

sol!(
    #[sol(rpc)]
    Challenge,
    "../lib/paradigm-ctf-2023/dai-plus-plus/challenge/project/out/Challenge.sol/Challenge.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Dai Plus Plus"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs
            .ethereum
            .paradigm_2023
            .dai_plus_plus
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?
            .address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        // Mirror the upstream Deploy script. The canonical ETH/USD feed is
        // planted as a fixed-price mock (see anvil::plant_eth_usd_feed).
        plant_eth_usd_feed(roles).await?;

        let configuration = SystemConfiguration::deploy(provider).await?;
        let manager =
            AccountManager::deploy(provider, *configuration.address()).await?;
        configuration
            .updateAccountManager(*manager.address())
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        let stablecoin =
            Stablecoin::deploy(provider, *configuration.address()).await?;
        configuration
            .updateStablecoin(*stablecoin.address())
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        let account_impl = AccountImpl::deploy(provider).await?;
        configuration
            .updateAccountImplementation(*account_impl.address())
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        configuration
            .updateEthUsdPriceFeed(crate::anvil::MAINNET_ETH_USD_FEED)
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        configuration
            .updateSystemContract(*manager.address(), true)
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        let contract =
            Challenge::deploy(provider, *configuration.address()).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = Challenge::new(self.address, provider.clone());

        let solved = contract.isSolved().call().await?;
        Ok(solved)
    }
}
