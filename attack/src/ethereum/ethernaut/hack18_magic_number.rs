//! Solution scaffold for Ethernaut level 18 — magic_number.
//!
//! Deploy 10-byte runtime returning 42.

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl18_magic_number::Target;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        // TODO: exploit the level
        let _ = (target, offender);
        Ok(())
    }
}
