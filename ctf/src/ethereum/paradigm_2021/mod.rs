//! Paradigm CTF 2021 challenges.
//!
//! Each challenge deploys a Setup contract that internally deploys the
//! challenge contracts and exposes `isSolved()`.
//!
//! NOTE: most 2021 setups wrap the canonical mainnet WETH9 at
//! `0xC02aaA…`; the harness plants a real WETH9 there via `anvil_setCode`
//! first (see [`crate::anvil::plant_weth9`]). broker/swap/farmer additionally
//! hardcode UniswapV2 factory/router plus DAI/COMP/cTokens — those stay
//! undeployed (`None`) until equivalent planting exists.

pub mod babyrev;
pub mod babysandbox;
pub mod bank;
pub mod bouncer;
pub mod broker;
pub mod farmer;
pub mod hello;
pub mod jop;
pub mod lockbox;
pub mod market;
pub mod rever;
pub mod secure;
pub mod swap;
pub mod upgrade;
pub mod vault;
pub mod yield_aggregator;

use crate::{Level as _, Roles};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Paradigm2021 {
    pub hello: hello::Target,
    /// upgrade: requires the real mainnet USDC proxy (fork-only)
    pub upgrade: Option<upgrade::Target>,
    // Remaining challenges: ABI + level modules ready, deploy TODO
    pub babyrev: Option<babyrev::Target>,
    pub babysandbox: Option<babysandbox::Target>,
    pub bank: Option<bank::Target>,
    pub bouncer: Option<bouncer::Target>,
    pub broker: Option<broker::Target>,
    pub farmer: Option<farmer::Target>,
    pub jop: Option<jop::Target>,
    pub lockbox: Option<lockbox::Target>,
    pub market: Option<market::Target>,
    pub rever: Option<rever::Target>,
    pub secure: Option<secure::Target>,
    pub swap: Option<swap::Target>,
    pub vault: Option<vault::Target>,
    pub yield_aggregator: Option<yield_aggregator::Target>,
}

pub async fn set_up_paradigm_2021(roles: &Roles) -> eyre::Result<Paradigm2021> {
    println!("Deploying Paradigm CTF 2021 challenges...");
    let hello = hello::Target::set_up(roles).await?;

    // broker/swap/farmer need planted Uniswap factory/router + stablecoins;
    // everything else is self-contained or only needs canonical WETH9.
    Ok(Paradigm2021 {
        hello,
        bank: bank::Target::set_up(roles).await.ok(),
        bouncer: bouncer::Target::set_up(roles).await.ok(),
        broker: None,
        farmer: None,
        lockbox: lockbox::Target::set_up(roles).await.ok(),
        babyrev: babyrev::Target::set_up(roles).await.ok(),
        babysandbox: babysandbox::Target::set_up(roles).await.ok(),
        jop: jop::Target::set_up(roles).await.ok(),
        rever: rever::Target::set_up(roles).await.ok(),
        upgrade: None,
        market: market::Target::set_up(roles).await.ok(),
        secure: secure::Target::set_up(roles).await.ok(),
        swap: None,
        vault: vault::Target::set_up(roles).await.ok(),
        yield_aggregator: yield_aggregator::Target::set_up(roles).await.ok(),
    })
}
