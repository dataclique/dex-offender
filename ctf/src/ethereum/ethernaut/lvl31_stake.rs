//! Ethernaut 31 — Stake.
//!
//! Goal: Leave `Stake` with nonzero Ether and `totalStaked` greater than that
//! balance. Your offender account must be recorded in `Stakers` but have zero
//! `UserStake`.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Stake.sol

use crate::{Level, Roles};
use alloy::{primitives::Address, primitives::U256, providers::Provider, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    Stake,
    "../lib/ethernaut/contracts/out/Stake.sol/Stake.json"
);

sol!(
    #[sol(rpc)]
    Weth,
    "../lib/damn-vulnerable-defi/out/WETH.sol/WETH.json"
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

        // Mirror StakeFactory: deploy a dummy WETH token (any ERC20
        // answering allowance/transferFrom works), then the Stake pool.
        let weth = Weth::deploy(provider).await?;
        let contract = Stake::deploy(provider, *weth.address()).await?;

        println!("Deployed the Stake contract at {}", contract.address());
        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();

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
