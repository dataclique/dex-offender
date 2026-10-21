//! Ethernaut level 22 — Dex.
//!
//! Drain both tokens from the Dex by exploiting the price calculation.

use crate::{Level, Roles};
use alloy::{primitives::Address, primitives::U256, providers::Provider, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Dex,
    "abi/ethernaut/Dex.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Dex"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level22.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let offender = &roles.offender;
        let provider = roles.provider();

        println!("Deploying the Dex contract...");
        let contract = Dex::deploy(provider).await?;

        // TODO: deploy SwappableToken instances and set up the pool
        // The old ethers implementation deployed two SwappableToken contracts,
        // approved them, added liquidity, and transferred 10 of each to the
        // offender. Needs SwappableToken ABI artifact.
        let _ = (deployer, offender);

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = Dex::new(self.address, provider.clone());

        println!("Checking that you have stolen all tokens of both types...");
        // Check both token balances of the Dex contract are zero
        let token1 = contract.token1().call().await?;
        let token2 = contract.token2().call().await?;

        // Use raw ERC20 balanceOf calls via the provider
        let balance_of_selector =
            alloy::primitives::keccak256("balanceOf(address)".as_bytes());
        let calldata1 =
            [&balance_of_selector[..4], &[0u8; 12] as &[u8], token1.as_slice()]
                .concat();
        let calldata2 =
            [&balance_of_selector[..4], &[0u8; 12] as &[u8], token2.as_slice()]
                .concat();

        let tx1 = alloy::rpc::types::TransactionRequest::default()
            .to(self.address)
            .input(alloy::primitives::Bytes::from(calldata1).into());
        let tx2 = alloy::rpc::types::TransactionRequest::default()
            .to(self.address)
            .input(alloy::primitives::Bytes::from(calldata2).into());

        let result1 = provider.call(tx1.clone()).await?;
        let result2 = provider.call(tx2).await?;

        let b1 = U256::try_from_be_slice(&result1[..32]).unwrap_or_default();
        let b2 = U256::try_from_be_slice(&result2[..32]).unwrap_or_default();

        Ok(b1 == U256::ZERO && b2 == U256::ZERO)
    }
}
