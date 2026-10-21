//! Ethernaut level 03 — Coin Flip.
//!
//! Predict the coin flip outcome 10 times in a row by predicting the
//! blockhash-based randomness.

use crate::{Level, Roles};
use alloy::{primitives::Address, primitives::U256, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    CoinFlip,
    "abi/ethernaut/CoinFlip.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Coin Flip"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level03.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the CoinFlip contract...");
        let contract = CoinFlip::deploy(provider).await?;

        let consecutive_wins = contract.consecutiveWins().call().await?;
        assert_eq!(consecutive_wins, U256::ZERO);

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = CoinFlip::new(self.address, provider.clone());

        println!("Checking that you won 10 times in a row...");
        let consecutive_wins = contract.consecutiveWins().call().await?;
        Ok(consecutive_wins >= U256::from(10))
    }
}
