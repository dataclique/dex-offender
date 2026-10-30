//! Ethernaut 27 — Good Samaritan.
//!
//! Goal: Drain all `Coin` tokens from the `Wallet` managed by `GoodSamaritan`.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/GoodSamaritan.sol

use crate::{Level, Roles};
use alloy::{primitives::Address, primitives::U256, providers::Provider, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    GoodSamaritan,
    "../lib/ethernaut/contracts/out/GoodSamaritan.sol/GoodSamaritan.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "GoodSamaritan"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level27.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the GoodSamaritan contract...");
        let contract = GoodSamaritan::deploy(provider).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = GoodSamaritan::new(self.address, provider.clone());

        // The wallet is drained when the coin balance of the wallet is zero
        let coin = contract.coin().call().await?;
        let wallet = contract.wallet().call().await?;

        // Read the wallet's coin balance via eth_call to balances(wallet)
        // — the Coin contract exposes a public mapping, not balanceOf.
        let selector =
            alloy::primitives::keccak256("balances(address)".as_bytes());
        let calldata =
            [&selector[..4], &[0u8; 12] as &[u8], wallet.as_slice()].concat();
        let tx = alloy::rpc::types::TransactionRequest::default()
            .to(coin)
            .input(alloy::primitives::Bytes::from(calldata).into());
        let result = provider.call(tx).await?;
        let balance =
            U256::try_from_be_slice(&result[..32]).unwrap_or_default();

        Ok(balance == U256::ZERO)
    }
}
