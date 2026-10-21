//! Ethernaut level 35 — Elliptic Token.
//!
//! Steve the $ETK tokens that ALICE just redeemed.

use crate::{Level, Roles};
use alloy::{primitives::Address, primitives::U256, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    EllipticToken,
    "abi/ethernaut/EllipticToken.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Elliptic Token"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethernaut.level35.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        println!("Deploying the EllipticToken contract...");
        let contract = EllipticToken::deploy(provider).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = EllipticToken::new(self.address, provider.clone());

        println!("Checking that you stole ALICE's tokens...");
        // ALICE = 0xA11CE84AcB91Ac59B0A4E2945C9157eF3Ab17D4e
        let alice = alloy::primitives::address!(
            "A11CE84AcB91Ac59B0A4E2945C9157eF3Ab17D4e"
        );
        let alice_balance = contract.balanceOf(alice).call().await?;
        Ok(alice_balance == U256::ZERO)
    }
}
