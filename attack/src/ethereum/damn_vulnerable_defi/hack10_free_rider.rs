//! Solution scaffold for DVD — FreeRider.
//!
//! Buy all NFTs for the price of one using a flash loan.

use ctf::Actor;
use ctf::ethereum::damn_vulnerable_defi::free_rider::Target;

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
