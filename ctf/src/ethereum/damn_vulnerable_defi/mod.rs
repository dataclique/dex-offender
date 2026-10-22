//! Damn Vulnerable DeFi challenges on the alloy core.
//!
//! Each challenge deploys its own set of contracts per the upstream
//! test files in lib/damn-vulnerable-defi/test/.

pub mod abi_smuggling;
pub mod backdoor;
pub mod climber;
pub mod compromised;
pub mod free_rider;
pub mod naive_receiver;
pub mod puppet;
pub mod puppet_v2;
pub mod puppet_v3;
pub mod selfie;
pub mod shards;
pub mod side_entrance;
pub mod the_rewarder;
pub mod truster;
pub mod unstoppable;
pub mod wallet_mining;
pub mod withdrawal;

use crate::{Level as _, Roles};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DamnVulnerableDeFi {
    pub unstoppable: unstoppable::Target,
    pub naive_receiver: naive_receiver::Target,
    pub truster: truster::Target,
    pub side_entrance: side_entrance::Target,
    pub the_rewarder: the_rewarder::Target,
    pub selfie: selfie::Target,
    pub compromised: compromised::Target,
    pub puppet: puppet::Target,
    pub puppet_v2: puppet_v2::Target,
    pub free_rider: free_rider::Target,
    pub backdoor: backdoor::Target,
    pub climber: climber::Target,
    // wallet_mining: set_up requires Safe factory + CreateX raw tx deploy (TODO)
    pub wallet_mining: Option<wallet_mining::Target>,
    // puppet_v3: requires mainnet fork at block 15450164
    pub puppet_v3: Option<puppet_v3::Target>,
    pub abi_smuggling: abi_smuggling::Target,
    pub shards: shards::Target,
    pub withdrawal: withdrawal::Target,
}

pub async fn set_up_damn_vulnerable_defi(
    roles: &Roles,
) -> eyre::Result<DamnVulnerableDeFi> {
    println!("Deploying Damn Vulnerable DeFi challenges...");
    let unstoppable = unstoppable::Target::set_up(roles).await?;
    let naive_receiver = naive_receiver::Target::set_up(roles).await?;
    let truster = truster::Target::set_up(roles).await?;
    let side_entrance = side_entrance::Target::set_up(roles).await?;
    let the_rewarder = the_rewarder::Target::set_up(roles).await?;
    let selfie = selfie::Target::set_up(roles).await?;
    let compromised = compromised::Target::set_up(roles).await?;
    let puppet = puppet::Target::set_up(roles).await?;
    let puppet_v2 = puppet_v2::Target::set_up(roles).await?;
    let free_rider = free_rider::Target::set_up(roles).await?;
    let backdoor = backdoor::Target::set_up(roles).await?;
    let climber = climber::Target::set_up(roles).await?;
    let abi_smuggling = abi_smuggling::Target::set_up(roles).await?;
    let shards = shards::Target::set_up(roles).await?;
    let withdrawal = withdrawal::Target::set_up(roles).await?;

    Ok(DamnVulnerableDeFi {
        unstoppable,
        naive_receiver,
        truster,
        side_entrance,
        the_rewarder,
        selfie,
        compromised,
        puppet,
        puppet_v2,
        free_rider,
        backdoor,
        climber,
        wallet_mining: None, // TODO: Safe factory + CreateX raw tx deploy
        puppet_v3: None,     // TODO: mainnet fork required
        abi_smuggling,
        shards,
        withdrawal,
    })
}
