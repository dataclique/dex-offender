//! Solution scaffold for DVD — PuppetV2.
//!
//! Goal: Empty the lending pool and send 1,000,000 DVT to the recovery
//! account (target.recovery).
//!
//! Contracts:
//! https://github.com/theredguild/damn-vulnerable-defi/blob/64fddf9f96de2782f8868898d68673acb295119c/src/puppet-v2/PuppetV2Pool.sol
//! https://github.com/theredguild/damn-vulnerable-defi/blob/64fddf9f96de2782f8868898d68673acb295119c/src/puppet-v2/UniswapV2Library.sol
//! https://github.com/theredguild/damn-vulnerable-defi/blob/64fddf9f96de2782f8868898d68673acb295119c/src/DamnValuableToken.sol

use ctf::Actor;
use ctf::ethereum::damn_vulnerable_defi::puppet_v2::*;

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
