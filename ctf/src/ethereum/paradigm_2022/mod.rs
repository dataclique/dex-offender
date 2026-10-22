//! Paradigm CTF 2022 challenges.
//!
//! Each challenge deploys a Setup contract that exposes `isSolved()`.
//!
//! NOTE: trapdooor/trapdoooor deploy raw bytecode (no Setup.sol with
//! isSolved) — they need custom deployment logic.

pub mod electric_sheep;
pub mod fun_reversing_challenge;
pub mod hint_finance;
pub mod just_in_time;
pub mod lockbox2;
pub mod merkledrop;
pub mod random;
pub mod rescue;
pub mod sourcecode;
pub mod stealing_sats;
pub mod vanity;

use crate::Roles;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Paradigm2022 {
    // electric_sheep needs mainnet fork — TODO
    pub electric_sheep: Option<electric_sheep::Target>,
    pub fun_reversing_challenge: Option<fun_reversing_challenge::Target>,
    pub hint_finance: Option<hint_finance::Target>,
    pub just_in_time: Option<just_in_time::Target>,
    pub lockbox2: Option<lockbox2::Target>,
    pub merkledrop: Option<merkledrop::Target>,
    pub random: Option<random::Target>,
    pub rescue: Option<rescue::Target>,
    pub sourcecode: Option<sourcecode::Target>,
    pub stealing_sats: Option<stealing_sats::Target>,
    pub vanity: Option<vanity::Target>,
}

pub async fn set_up_paradigm_2022(roles: &Roles) -> eyre::Result<Paradigm2022> {
    println!("Deploying Paradigm CTF 2022 challenges...");
    // TODO: most 2022 Setup constructors revert on local anvil (similar to 2021)
    // Fix per-challenge deployment. See each module's set_up for the attempt.
    let _ = roles;

    Ok(Paradigm2022 {
        electric_sheep: None, // also needs mainnet fork
        fun_reversing_challenge: None,
        hint_finance: None,
        just_in_time: None,
        lockbox2: None,
        merkledrop: None,
        random: None,
        rescue: None,
        sourcecode: None,
        stealing_sats: None,
        vanity: None,
    })
}
