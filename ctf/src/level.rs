use crate::roles::*;
use async_trait::async_trait;

#[async_trait]
pub trait Level {
    fn name(&self) -> &'static str;

    fn from_file() -> eyre::Result<Self>
    where
        Self: Sized;

    async fn set_up(roles: &Roles) -> eyre::Result<Self>
    where
        Self: Sized;

    async fn check(&self, roles: &Roles) -> eyre::Result<bool>;
}

#[async_trait]
pub trait Exploit {
    type Target: Level;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()>;
}

pub async fn check_exploit<E: Exploit>(
    roles: &Roles,
    exploit: E,
) -> eyre::Result<()> {
    let level = E::Target::from_file()?;

    if level.check(roles).await? {
        println!("Already solved: {}", level.name());
        return Ok(());
    }
    println!("\n\n{}", level.name());

    println!("Running the exploit...");
    exploit.attack(&level, &roles.offender).await?;

    println!("Checking the exploit...");
    let solved = level.check(roles).await?;
    if !solved {
        return Err(eyre::eyre!(
            "exploit for {} ran without error but the level check still fails",
            level.name()
        ));
    }

    println!(
        "

-------------------------------------
//////     CONGRATULATIONS     //////

$$$  Y O U  H A V E  S O L V E D  $$$
$$$   T H E   C H A L L E N G E   $$$

youpassedthelevelyoupassedthelevelyou
passedthelevelyoupassedthelevelyoupas
sedthelevelyoupassedthelevelyoupassed
-------------------------------------
"
    );

    Ok(())
}
