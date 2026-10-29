//! Solution scaffold for DVD — Withdrawal.
//!
//! Finalize cross-chain withdrawals safely.

use ctf::Actor;
use ctf::ethereum::damn_vulnerable_defi::withdrawal::*;

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
