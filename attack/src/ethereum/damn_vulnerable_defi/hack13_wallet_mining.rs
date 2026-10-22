//! Solution scaffold for DVD — Wallet Mining.
//!
//! Deploy wallets without governance approval and drain the deployer.

use ctf::Actor;
use ctf::ethereum::damn_vulnerable_defi::wallet_mining::Target;

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
