//! Solution scaffold for DVD — ABI Smuggling.
//!
//! Exploit the selector clash to sweep the vault.

use ctf::Actor;
use ctf::ethereum::damn_vulnerable_defi::abi_smuggling::*;

pub struct Exploit;

#[async_trait::async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        let _ = (target, offender);
        // TODO: exploit the level
        Ok(())
    }
}
