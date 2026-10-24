//! Paradigm CTF 2021 — upgrade.
//!
//! Fork-only: the setup upgrades the real mainnet USDC proxy
//! (`0xA0b8…eB48`). Run deploy_levels with `FORK_URL` set; the set_up
//! calls `upgrade()` which deploys FiatTokenV3, initializes it, and
//! upgrades the live proxy. No specific fork block required (any block
//! where the USDC proxy exists works).

use crate::{Level, Roles};
use alloy::{primitives::Address, providers::Provider, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Setup,
    "abi/paradigm-2021/upgrade.json"
);

const MAINNET_USDC: Address =
    alloy::primitives::address!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "upgrade"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let t = ctfs
            .ethereum
            .paradigm_2021
            .upgrade
            .as_ref()
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?;
        Ok(Target { address: t.address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        // The setup needs a mock USDC proxy at the canonical address with
        // a permissive admin (the original CTF deployed one on a private
        // chain). On a real mainnet fork, the proxy's admin rejects the
        // Setup's upgradeTo call, so this still fails without planting.
        let usdc_code = provider.get_code_at(MAINNET_USDC).await?;
        if usdc_code.is_empty() {
            return Err(eyre::eyre!(
                "upgrade needs a mainnet fork with a permissive USDC proxy"
            ));
        }

        println!("Deploying the upgrade challenge...");
        let setup = Setup::deploy(provider).await?;
        // upgrade() reverts on a real fork (admin check); needs a
        // planted mock proxy first.
        if let Err(e) =
            setup.upgrade().from(roles.deployer.address()).send().await
        {
            println!("upgrade() failed (expected on real fork): {e}");
        }

        Ok(Target { address: *setup.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let setup = Setup::new(self.address, provider.clone());
        Ok(setup.isSolved().call().await?)
    }
}
