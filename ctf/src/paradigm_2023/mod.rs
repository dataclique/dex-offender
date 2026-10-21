//! Paradigm CTF 2023 challenges on the alloy core.
//!
//! Each challenge deploys a Challenge contract (and optional helpers).
//! The `isSolved()` method on each Challenge contract is the completion
//! check.

pub mod black_sheep;
pub mod cosmic_radiation;
pub mod dai_plus_plus;
pub mod dodont;
pub mod dragon_tyrant;
pub mod dropper;
pub mod enterprise_blockchain;
pub mod free_real_estate;
pub mod grains_of_sand;
pub mod hello_world;
pub mod hopping_into_place;
pub mod one_hundred_percent;
pub mod skill_based_game;
pub mod suspicious_charity;

use crate::{Level as _, Roles};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Paradigm2023 {
    pub hello_world: hello_world::Target,
    pub cosmic_radiation: cosmic_radiation::Target,
    pub grains_of_sand: grains_of_sand::Target,
    pub black_sheep: black_sheep::Target,
    pub dai_plus_plus: dai_plus_plus::Target,
    pub dodont: dodont::Target,
    pub dragon_tyrant: dragon_tyrant::Target,
    pub dropper: dropper::Target,
    pub enterprise_blockchain: enterprise_blockchain::Target,
    pub free_real_estate: free_real_estate::Target,
    pub hopping_into_place: hopping_into_place::Target,
    pub one_hundred_percent: one_hundred_percent::Target,
    pub skill_based_game: skill_based_game::Target,
    pub suspicious_charity: suspicious_charity::Target,
}

pub async fn set_up_paradigm_2023(roles: &Roles) -> eyre::Result<Paradigm2023> {
    println!("Deploying Paradigm CTF 2023 challenges...");
    let hello_world = hello_world::Target::set_up(roles).await?;
    let cosmic_radiation = cosmic_radiation::Target::set_up(roles).await?;
    let grains_of_sand = grains_of_sand::Target::set_up(roles).await?;
    let black_sheep = black_sheep::Target::set_up(roles).await?;
    let dai_plus_plus = dai_plus_plus::Target::set_up(roles).await?;
    let dodont = dodont::Target::set_up(roles).await?;
    let dragon_tyrant = dragon_tyrant::Target::set_up(roles).await?;
    let dropper = dropper::Target::set_up(roles).await?;
    let enterprise_blockchain =
        enterprise_blockchain::Target::set_up(roles).await?;
    let free_real_estate = free_real_estate::Target::set_up(roles).await?;
    let hopping_into_place = hopping_into_place::Target::set_up(roles).await?;
    let one_hundred_percent =
        one_hundred_percent::Target::set_up(roles).await?;
    let skill_based_game = skill_based_game::Target::set_up(roles).await?;
    let suspicious_charity = suspicious_charity::Target::set_up(roles).await?;

    Ok(Paradigm2023 {
        hello_world,
        cosmic_radiation,
        grains_of_sand,
        black_sheep,
        dai_plus_plus,
        dodont,
        dragon_tyrant,
        dropper,
        enterprise_blockchain,
        free_real_estate,
        hopping_into_place,
        one_hundred_percent,
        skill_based_game,
        suspicious_charity,
    })
}
