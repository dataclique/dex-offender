//! Solution scaffold for DVD — Puppet.
//!
//! Goal: Empty the lending pool and send its 100,000 DVT to the recovery
//! account (target.recovery) in ONE transaction.
//!
//! Contracts:
//! https://github.com/theredguild/damn-vulnerable-defi/blob/64fddf9f96de2782f8868898d68673acb295119c/src/puppet/PuppetPool.sol
//! https://github.com/theredguild/damn-vulnerable-defi/blob/64fddf9f96de2782f8868898d68673acb295119c/src/puppet/IUniswapV1Factory.sol
//! https://github.com/theredguild/damn-vulnerable-defi/blob/64fddf9f96de2782f8868898d68673acb295119c/src/puppet/IUniswapV1Exchange.sol
//! https://github.com/theredguild/damn-vulnerable-defi/blob/64fddf9f96de2782f8868898d68673acb295119c/src/DamnValuableToken.sol

use ctf::Actor;
use ctf::ethereum::damn_vulnerable_defi::puppet::*;

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
