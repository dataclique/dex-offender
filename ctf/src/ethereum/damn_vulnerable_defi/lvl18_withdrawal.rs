//! Damn Vulnerable DeFi level 18 — Withdrawal
//!
//! There's a token bridge to withdraw Damn Valuable Tokens from an L2 to an L1.
//! It has a million DVT tokens in balance.
//!
//! The L1 side of the bridge allows anyone to finalize withdrawals, as long as
//! the delay period has passed and they present a valid Merkle proof. The proof
//! must correspond with the latest withdrawals' root set by the bridge owner.
//!
//! You were given the event logs of 4 withdrawals initiated on L2 in a JSON
//! file. They could be executed on L1 after the 7 days delay.
//!
//! But there's one suspicious among them, isn't there? You may want to
//! double-check, because all funds might be at risk. Luckily you are a bridge
//! operator with special powers.
//!
//! Protect the bridge by finalizing all given withdrawals, preventing the
//! suspicious one from executing, and somehow not draining all funds.
//!
//! Full description: lib/damn-vulnerable-defi/src/withdrawal/README.md
//! (pinned submodule, v4.1.0).

// TODO: implement deployment and the completion check, then declare this
// module in `ctf/src/damn_vulnerable_defi/mod.rs` and re-enable the
// `damn_vulnerable_defi` module in `ctf/src/lib.rs` when you want it deployed.

use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::l1_gateway::L1Gateway;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethereum.damn_vulnerable_defi.level18)
    }

    fn name(&self) -> &'static str {
        "Withdrawal"
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the L1Gateway contract...");
        // TODO: deploy the level contracts (L1 + L2 sides) and fund the
        // offender.
        let _ = deployer;
        let target = Target { address: Address::zero() };

        let check = target.check(roles).await?;
        assert!(!check);

        Ok(target)
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        // TODO: check the completion condition for this level.
        let _ = roles;
        Ok(false)
    }
}
