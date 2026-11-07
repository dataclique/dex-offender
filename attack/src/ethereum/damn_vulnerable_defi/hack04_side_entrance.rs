//! Solution scaffold for DVD — Side Entrance.
//!
//! Goal: Empty the pool and send its 1,000 ETH to the recovery account
//! (target.recovery).
//!
//! Contract (including its borrower interface):
//! https://github.com/theredguild/damn-vulnerable-defi/blob/64fddf9f96de2782f8868898d68673acb295119c/src/side-entrance/SideEntranceLenderPool.sol

use ctf::Actor;
use ctf::ethereum::damn_vulnerable_defi::side_entrance::*;

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
