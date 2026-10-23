//! Paradigm CTF 2022 challenges.
//!
//! Each challenge deploys a Setup contract that exposes `isSolved()`.
//!
//! NOTE: trapdooor/trapdoooor are off-chain judged challenges — the
//! player submits runtime bytecode that factors a random semiprime,
//! graded by a forge script (see their public/deploy/chal.py). There is
//! no on-chain Setup, so they have no harness module.

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

use crate::{Level as _, Roles};

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
    // Each set_up self-documents its own requirements; failures fall back
    // to None so one broken challenge never blocks the whole deployment.
    // Fork-only: electric_sheep (NFT state), hint_finance (mainnet tokens),
    // stealing_sats (mainnet WBTC + Bitfinex wallet).

    Ok(Paradigm2022 {
        electric_sheep: None,
        fun_reversing_challenge: fun_reversing_challenge::Target::set_up(roles)
            .await
            .ok(),
        hint_finance: None,
        just_in_time: just_in_time::Target::set_up(roles).await.ok(),
        lockbox2: lockbox2::Target::set_up(roles).await.ok(),
        merkledrop: merkledrop::Target::set_up(roles).await.ok(),
        random: random::Target::set_up(roles).await.ok(),
        rescue: None, // fork-only: real SushiSwap MasterChef + router
        sourcecode: sourcecode::Target::set_up(roles).await.ok(),
        stealing_sats: None,
        vanity: vanity::Target::set_up(roles).await.ok(),
    })
}
