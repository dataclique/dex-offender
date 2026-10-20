//! Ethernaut level 35 — Elliptic Token
//!
//! BOB created and owns a new ERC20 token with an elliptic curve–based signed
//! voucher redemption system called EllipticToken ($ETK). Bob can create
//! vouchers off-chain that can be redeemed on-chain for $ETK. The contract also
//! includes a permit system based on elliptic curve signatures.
//!
//! Bob is a lazy developer and "optimized" some steps of the ECDSA algorithm.
//! Can you find the flaw?
//!
//! Your goal is to steal the $ETK tokens that ALICE
//! (`0xA11CE84AcB91Ac59B0A4E2945C9157eF3Ab17D4e`) just redeemed.
//!
//! Things that might help:
//! - Look for any missing step in the Elliptic Curve Digital Signature
//!   Algorithm (ECDSA).
//!
//! Good luck. Elliptic curves do not forgive domain confusion.

// TODO: implement deployment and the completion check, then declare this
// module in `ctf/src/ethernaut/mod.rs` (add the `level35` field to the
// `Ethernaut` struct and the `use` alias) when you want it deployed.

use crate::{roles::*, Level};
use async_trait::async_trait;
use ethers::prelude::*;

pub use crate::abi::elliptic_token::EllipticToken;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethernaut.level35)
    }

    fn name(&self) -> &'static str {
        "Elliptic Token"
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let Roles { deployer, offender: _, some_user: _ } = roles;

        println!("Deploying the EllipticToken contract...");
        // TODO: deploy the level contracts and fund the offender.
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
