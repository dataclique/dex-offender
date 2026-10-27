//! Ethernaut level 05 — Token.
//!
//! Goal: Increase your offender account's `Token` balance above the twenty tokens it receives at setup.
//!
//! Source: [local](../../../../lib/ethernaut/contracts/src/levels/Token.sol) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/contracts/src/levels/Token.sol).
//! Challenge: [local](../../../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/token.md) · [pinned upstream](https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/client/src/gamedata/en/descriptions/levels/token.md).
//! [Factory success check](../../../../lib/ethernaut/contracts/src/levels/TokenFactory.sol) · [Rust bindings](crate::ethereum::ethernaut::lvl05_token).
//!
//! `target` contains the deployed instance address and any level-specific helpers.
//! `offender` is your player account; sign transactions with `offender.address()`.

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
