//! Ethernaut level 23 — DexTwo.
//!
//! Drain both tokens from DexTwo by adding a fake token to the pool.

use crate::{Level, Roles};
use alloy::{primitives::Address, primitives::U256, providers::Provider, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    DexTwo,
    "abi/ethernaut/DexTwo.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "DexTwo"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethernaut.level23.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let offender = &roles.offender;
        let provider = roles.provider();

        println!("Deploying the DexTwo contract...");
        let contract = DexTwo::deploy(provider).await?;

        // TODO: deploy SwappableTokenTwo instances and set up the pool
        let _ = (deployer, offender);

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = DexTwo::new(self.address, provider.clone());

        println!("Checking that you have stolen all tokens of both types...");
        let token1 = contract.token1().call().await?;
        let token2 = contract.token2().call().await?;

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
