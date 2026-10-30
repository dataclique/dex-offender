//! Ethernaut 23 — Dex Two.
//!
//! Goal: Drain both designated tokens from `DexTwo`. The DEX starts with 100 of
//! each token, and your offender account starts with ten of each.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/DexTwo.sol

use crate::{Level, Roles};
use alloy::{primitives::Address, primitives::U256, providers::Provider, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    DexTwo,
    "../lib/ethernaut/contracts/out/DexTwo.sol/DexTwo.json"
);

sol!(
    #[sol(rpc)]
    SwappableTokenTwo,
    "../lib/ethernaut/contracts/out/DexTwo.sol/SwappableTokenTwo.json"
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
        let address = ctfs.ethereum.ethernaut.level23.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let offender = &roles.offender;
        let provider = roles.provider();

        println!("Deploying the DexTwo contract...");
        let contract = DexTwo::deploy(provider).await?;
        let dex_address = *contract.address();

        // Mirror DexTwoFactory.createInstance (add_liquidity spelling).
        let token1 = SwappableTokenTwo::deploy(
            provider,
            dex_address,
            "Token 1".to_string(),
            "TKN1".to_string(),
            U256::from(110),
        )
        .await?;
        let token2 = SwappableTokenTwo::deploy(
            provider,
            dex_address,
            "Token 2".to_string(),
            "TKN2".to_string(),
            U256::from(110),
        )
        .await?;

        contract
            .setTokens(*token1.address(), *token2.address())
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        for token in [&token1, &token2] {
            token
                .approve_0(dex_address, U256::from(100))
                .from(deployer.address())
                .send()
                .await?
                .watch()
                .await?;
        }
        contract
            .add_liquidity(*token1.address(), U256::from(100))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        contract
            .add_liquidity(*token2.address(), U256::from(100))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        for token in [&token1, &token2] {
            token
                .transfer(offender.address(), U256::from(10))
                .from(deployer.address())
                .send()
                .await?
                .watch()
                .await?;
        }

        Ok(Target { address: dex_address })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = DexTwo::new(self.address, provider.clone());

        let token1 = contract.token1().call().await?;
        let token2 = contract.token2().call().await?;

        let balance_of_selector =
            alloy::primitives::keccak256("balanceOf(address)".as_bytes());
        let calldata1 = [
            &balance_of_selector[..4],
            &[0u8; 12] as &[u8],
            self.address.as_slice(),
        ]
        .concat();
        let calldata2 = [
            &balance_of_selector[..4],
            &[0u8; 12] as &[u8],
            self.address.as_slice(),
        ]
        .concat();

        let tx1 = alloy::rpc::types::TransactionRequest::default()
            .to(token1)
            .input(alloy::primitives::Bytes::from(calldata1).into());
        let tx2 = alloy::rpc::types::TransactionRequest::default()
            .to(token2)
            .input(alloy::primitives::Bytes::from(calldata2).into());

        let result1 = provider.call(tx1.clone()).await?;
        let result2 = provider.call(tx2).await?;

        let b1 = U256::try_from_be_slice(&result1[..32]).unwrap_or_default();
        let b2 = U256::try_from_be_slice(&result2[..32]).unwrap_or_default();

        Ok(b1 == U256::ZERO && b2 == U256::ZERO)
    }
}
