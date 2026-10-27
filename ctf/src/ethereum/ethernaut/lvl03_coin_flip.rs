//! Ethernaut level 03 — Coin Flip.
//!
//! Goal: Win at least ten consecutive guesses in the `CoinFlip` game without resetting the winning streak.
//!
//! Source: [local](../../../../lib/ethernaut/contracts/src/levels/CoinFlip.sol) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/contracts/src/levels/CoinFlip.sol).
//! Challenge: [local](../../../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/coinflip.md) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/client/src/gamedata/en/descriptions/levels/coinflip.md).
//! [Factory success check](../../../../lib/ethernaut/contracts/src/levels/CoinFlipFactory.sol) · [Rust bindings](crate::ethereum::ethernaut::lvl03_coin_flip).
//!
//! `target` contains the deployed instance address and any level-specific helpers.
//! `offender` is your player account; sign transactions with `offender.address()`.

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
