//! Ethernaut 05 — Token.
//!
//! Goal: Increase your offender account's `Token` balance above the twenty
//! tokens it receives at setup.
//!
//! Source: [Token.sol][source]. [Rust harness][harness].
//!
//! [source]:
//! ../../../../lib/ethernaut/contracts/src/levels/Token.sol
//! [harness]: crate::ethereum::ethernaut::lvl05_token

use crate::{Level, Roles};
use alloy::{primitives::Address, primitives::U256, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Token,
    "abi/ethernaut/Token.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Token"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level05.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let offender = &roles.offender;
        let provider = roles.provider();

        println!("Deploying the Token contract...");
        let contract = Token::deploy(provider, U256::from(21_000_000)).await?;

        contract
            .transfer(offender.address(), U256::from(20))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = Token::new(self.address, provider.clone());

        println!("Checking that you got more tokens...");
        let balance =
            contract.balanceOf(roles.offender.address()).call().await?;
        Ok(balance > U256::from(20))
    }
}
