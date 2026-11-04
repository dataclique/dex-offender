//! Solution scaffold for DVD — Withdrawal.
//!
//! Goal: Finalize all four supplied withdrawals while retaining more
//! than 990,000 but less than 1,000,000 DVT in the bridge and leaving
//! the player with no DVT.
//!
//! Contracts:
//! https://github.com/theredguild/damn-vulnerable-defi/blob/64fddf9f96de2782f8868898d68673acb295119c/src/withdrawal/L1Gateway.sol
//! https://github.com/theredguild/damn-vulnerable-defi/blob/64fddf9f96de2782f8868898d68673acb295119c/src/withdrawal/L1Forwarder.sol
//! https://github.com/theredguild/damn-vulnerable-defi/blob/64fddf9f96de2782f8868898d68673acb295119c/src/withdrawal/L2Handler.sol
//! https://github.com/theredguild/damn-vulnerable-defi/blob/64fddf9f96de2782f8868898d68673acb295119c/src/withdrawal/L2MessageStore.sol
//! https://github.com/theredguild/damn-vulnerable-defi/blob/64fddf9f96de2782f8868898d68673acb295119c/src/withdrawal/TokenBridge.sol

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
