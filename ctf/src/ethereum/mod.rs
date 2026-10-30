//! Ethereum-chain CTFs: Ethernaut, Damn Vulnerable DeFi, Paradigm CTF.

pub mod damn_vulnerable_defi;
pub mod ethernaut;
#[cfg(feature = "paradigm-legacy")]
pub mod paradigm_2021;
#[cfg(feature = "paradigm-legacy")]
pub mod paradigm_2022;
pub mod paradigm_2023;

use crate::Roles;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Ethereum {
    pub ethernaut: ethernaut::Ethernaut,
    pub paradigm_2023: paradigm_2023::Paradigm2023,
    #[cfg(feature = "paradigm-legacy")]
    pub paradigm_2021: paradigm_2021::Paradigm2021,
    #[cfg(feature = "paradigm-legacy")]
    pub paradigm_2022: paradigm_2022::Paradigm2022,
    pub damn_vulnerable_defi: damn_vulnerable_defi::DamnVulnerableDeFi,
}

pub async fn set_up_ethereum(roles: &Roles) -> eyre::Result<Ethereum> {
    let ethernaut = ethernaut::set_up_ethernaut(roles).await?;
    let paradigm_2023 = paradigm_2023::set_up_paradigm_2023(roles).await?;
    #[cfg(feature = "paradigm-legacy")]
    let paradigm_2021 = paradigm_2021::set_up_paradigm_2021(roles).await?;
    #[cfg(feature = "paradigm-legacy")]
    let paradigm_2022 = paradigm_2022::set_up_paradigm_2022(roles).await?;
    let damn_vulnerable_defi =
        damn_vulnerable_defi::set_up_damn_vulnerable_defi(roles).await?;

    Ok(Ethereum {
        ethernaut,
        paradigm_2023,
        #[cfg(feature = "paradigm-legacy")]
        paradigm_2021,
        #[cfg(feature = "paradigm-legacy")]
        paradigm_2022,
        damn_vulnerable_defi,
    })
}
