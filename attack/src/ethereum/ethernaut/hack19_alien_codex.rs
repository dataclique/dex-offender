//! Solution scaffold for Ethernaut level 19 — alien_codex.
//!
//! Underflow array length to write owner slot.

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl19_alien_codex::Target;

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
