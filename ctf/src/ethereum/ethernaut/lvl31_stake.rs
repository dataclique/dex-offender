//! Ethernaut level 31 — Stake.
//!
//! Drain the Stake contract (native ETH + WETH staking).

use crate::{Level, Roles};
use alloy::{primitives::Address, primitives::U256, providers::Provider, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Stake,
    "abi/ethernaut/Stake.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Stake"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level31.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the Stake contract...");
        // Stake takes a WETH address
        // TODO: deploy WETH first
        let contract = Stake::deploy(provider, Address::ZERO).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();

        println!("Checking the staking conditions...");
        // Solved when: ETH balance > 0, totalStaked > balance,
        // you're a staker, and your stake is 0
        let contract = Stake::new(self.address, provider.clone());
        let eth_balance = provider.get_balance(self.address).await?;
        let total_staked = contract.totalStaked().call().await?;
        let is_staker =
            contract.Stakers(roles.offender.address()).call().await?;

        Ok(eth_balance > U256::ZERO && total_staked > eth_balance && is_staker)
    }
}
