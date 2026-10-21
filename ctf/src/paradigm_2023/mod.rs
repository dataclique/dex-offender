//! Paradigm CTF 2023 challenges on the alloy core.
//!
//! Each challenge deploys a Challenge contract (and optional helpers).
//! The `isSolved()` method on each Challenge contract is the completion
//! check.

pub mod hello_world;

use crate::{Level, Roles};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Paradigm2023 {
    pub hello_world: hello_world::Target,
}

pub async fn set_up_paradigm_2023(roles: &Roles) -> eyre::Result<Paradigm2023> {
    println!("Deploying Paradigm CTF 2023 challenges...");
    let hello_world = hello_world::Target::set_up(roles).await?;

    Ok(Paradigm2023 { hello_world })
}
