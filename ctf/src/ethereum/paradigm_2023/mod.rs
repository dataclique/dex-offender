//! Paradigm CTF 2023 challenges on the alloy core.
//!
//! Challenges are deployed as their set_up is implemented. `None` fields
//! mean the challenge is known but not yet deployable (needs helper
//! contracts, mainnet fork, or different completion check).

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
    /// Requires a mainnet fork (reads hardcoded mainnet token addresses)
    pub grains_of_sand: Option<grains_of_sand::Target>,
    /// Needs ISimpleBank helper contract deployed first
    pub black_sheep: Option<black_sheep::Target>,
    /// Needs SystemConfiguration helper deployed first
    pub dai_plus_plus: Option<dai_plus_plus::Target>,
    /// Needs DVM token helper deployed first
    pub dodont: Option<dodont::Target>,
    /// Needs Factory + ItemShop + NFT helpers deployed first
    pub dragon_tyrant: Option<dragon_tyrant::Target>,
    /// Uses getScore() instead of isSolved()
    pub dropper: Option<dropper::Target>,
    /// Needs Bridge + FlagToken helpers deployed first
    pub enterprise_blockchain: Option<enterprise_blockchain::Target>,
    /// Needs MerkleDistributor helper deployed first
    pub free_real_estate: Option<free_real_estate::Target>,
    /// Needs Bridge helper deployed first
    pub hopping_into_place: Option<hopping_into_place::Target>,
    /// Needs Split helper deployed first
    pub one_hundred_percent: Option<one_hundred_percent::Target>,
    /// Needs Blackjack helper deployed first
    pub skill_based_game: Option<skill_based_game::Target>,
    /// Needs Router helper deployed first
    pub suspicious_charity: Option<suspicious_charity::Target>,
}

pub async fn set_up_paradigm_2023(roles: &Roles) -> eyre::Result<Paradigm2023> {
    println!("Deploying Paradigm CTF 2023 challenges...");
    let hello_world = hello_world::Target::set_up(roles).await?;
    let cosmic_radiation = cosmic_radiation::Target::set_up(roles).await?;

    Ok(Paradigm2023 {
        hello_world,
        cosmic_radiation,
        grains_of_sand: None,
        black_sheep: None,
        dai_plus_plus: None,
        dodont: None,
        dragon_tyrant: None,
        dropper: None,
        enterprise_blockchain: None,
        free_real_estate: None,
        hopping_into_place: None,
        one_hundred_percent: None,
        skill_based_game: None,
        suspicious_charity: None,
    })
}
