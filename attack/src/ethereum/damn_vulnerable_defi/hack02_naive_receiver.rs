//! Solution scaffold for DVD — Naive Receiver.
//!
//! Goal: Empty the pool and the receiver, sending all 1,010 WETH to the
//! recovery account (target.recovery) in at most two transactions.
//!
//! Contracts:
//! https://github.com/theredguild/damn-vulnerable-defi/blob/64fddf9f96de2782f8868898d68673acb295119c/src/naive-receiver/NaiveReceiverPool.sol
//! https://github.com/theredguild/damn-vulnerable-defi/blob/64fddf9f96de2782f8868898d68673acb295119c/src/naive-receiver/FlashLoanReceiver.sol
//! https://github.com/theredguild/damn-vulnerable-defi/blob/64fddf9f96de2782f8868898d68673acb295119c/src/naive-receiver/BasicForwarder.sol
//! https://github.com/theredguild/damn-vulnerable-defi/blob/64fddf9f96de2782f8868898d68673acb295119c/src/naive-receiver/Multicall.sol

use ctf::Actor;
use ctf::ethereum::damn_vulnerable_defi::naive_receiver::*;

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
