//! Solution scaffold for Paradigm CTF 2023 — Hello World.
//!
//! Goal: Increase TARGET's ETH balance by strictly more than 13.37 ETH
//! above STARTING_BALANCE.
//!
//! Contract:
//! https://github.com/paradigmxyz/paradigm-ctf-2023/blob/642d6aa90a4ca195337e8cd5f09cb2f7f842a481/hello-world/challenge/project/src/Challenge.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::paradigm_2023::hello_world::*;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        let _ = (target, offender);
        // TODO: exploit the challenge
        Ok(())
    }
}
