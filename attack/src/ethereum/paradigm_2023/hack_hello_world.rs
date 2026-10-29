//! Solution scaffold for Paradigm CTF 2023 — Hello World.

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
