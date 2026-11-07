//! Solution scaffold for DVD — Compromised.
//!
//! Goal: Drain the exchange's 999 ETH to the recovery account
//! (target.recovery), holding no NFTs and leaving the oracle median
//! unchanged.
//!
//! Contracts:
//! https://github.com/theredguild/damn-vulnerable-defi/blob/64fddf9f96de2782f8868898d68673acb295119c/src/compromised/Exchange.sol
//! https://github.com/theredguild/damn-vulnerable-defi/blob/64fddf9f96de2782f8868898d68673acb295119c/src/compromised/TrustfulOracle.sol
//! https://github.com/theredguild/damn-vulnerable-defi/blob/64fddf9f96de2782f8868898d68673acb295119c/src/compromised/TrustfulOracleInitializer.sol
//! https://github.com/theredguild/damn-vulnerable-defi/blob/64fddf9f96de2782f8868898d68673acb295119c/src/DamnValuableNFT.sol

use ctf::Actor;
use ctf::ethereum::damn_vulnerable_defi::compromised::*;

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
