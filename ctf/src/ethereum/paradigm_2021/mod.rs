//! Paradigm CTF 2021 challenges.
//!
//! Each challenge deploys a Setup contract that internally deploys the
//! challenge contracts and exposes `isSolved()`.
//!
//! NOTE: most 2021 Setup constructors revert on local anvil due to old
//! solc versions and pre-Byzantium EVM semantics. Only `hello` currently
//! deploys. The remaining ABI artifacts and level modules are ready —
//! they just need per-challenge deploy fixes (vm.deal equivalents,
//! hardcoded WETH addresses, etc.).

pub mod bank;
pub mod bouncer;
pub mod broker;
pub mod farmer;
pub mod hello;
pub mod lockbox;
pub mod market;
pub mod secure;
pub mod swap;
pub mod vault;
pub mod yield_aggregator;

use crate::{Level as _, Roles};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Paradigm2021 {
    pub hello: hello::Target,
    // Remaining challenges: ABI + level modules ready, deploy TODO
    pub bank: Option<bank::Target>,
    pub bouncer: Option<bouncer::Target>,
    pub broker: Option<broker::Target>,
    pub farmer: Option<farmer::Target>,
    pub lockbox: Option<lockbox::Target>,
    pub market: Option<market::Target>,
    pub secure: Option<secure::Target>,
    pub swap: Option<swap::Target>,
    pub vault: Option<vault::Target>,
    pub yield_aggregator: Option<yield_aggregator::Target>,
}

pub async fn set_up_paradigm_2021(roles: &Roles) -> eyre::Result<Paradigm2021> {
    println!("Deploying Paradigm CTF 2021 challenges...");
    let hello = hello::Target::set_up(roles).await?;

    Ok(Paradigm2021 {
        hello,
        bank: None,
        bouncer: None,
        broker: None,
        farmer: None,
        lockbox: None,
        market: None,
        secure: None,
        swap: None,
        vault: None,
        yield_aggregator: None,
    })
}
