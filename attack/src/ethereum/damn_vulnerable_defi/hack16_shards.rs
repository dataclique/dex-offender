//! Solution scaffold for DVD — Shards.
//!
//! Buy the Damn Valuable NFT for 1% of its price.

use ctf::Actor;
use ctf::ethereum::damn_vulnerable_defi::shards::Target;

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
