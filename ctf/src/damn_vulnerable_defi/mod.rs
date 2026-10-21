//! Damn Vulnerable DeFi challenges on the alloy core.
//!
//! Each challenge deploys its own set of contracts per the upstream
//! test files in lib/damn-vulnerable-defi/test/.

// pub mod unstoppable;
pub mod unstoppable;

use crate::{Level as _, Roles};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DamnVulnerableDeFi {
    pub unstoppable: unstoppable::Target,
}

pub async fn set_up_damn_vulnerable_defi(
    roles: &Roles,
) -> eyre::Result<DamnVulnerableDeFi> {
    println!("Deploying Damn Vulnerable DeFi challenges...");
    let unstoppable = unstoppable::Target::set_up(roles).await?;

    Ok(DamnVulnerableDeFi { unstoppable })
}
