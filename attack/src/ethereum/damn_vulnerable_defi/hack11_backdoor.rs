//! Solution scaffold for DVD — Backdoor.
//!
//! Goal: Register wallets for all four beneficiaries and drain the
//! registry's 40 DVT to the designated recovery account (anvil account 4,
//! 0x90F8…b906) in ONE transaction.
//!
//! Contracts:
//! https://github.com/theredguild/damn-vulnerable-defi/blob/64fddf9f96de2782f8868898d68673acb295119c/src/backdoor/WalletRegistry.sol
//! https://github.com/safe-global/safe-smart-account/blob/bf943f80fec5ac647159d26161446ac5d716a294/contracts/Safe.sol
//! https://github.com/safe-global/safe-smart-account/blob/bf943f80fec5ac647159d26161446ac5d716a294/contracts/proxies/SafeProxyFactory.sol
//! https://github.com/theredguild/damn-vulnerable-defi/blob/64fddf9f96de2782f8868898d68673acb295119c/src/DamnValuableToken.sol

use ctf::Actor;
use ctf::ethereum::damn_vulnerable_defi::backdoor::*;

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
