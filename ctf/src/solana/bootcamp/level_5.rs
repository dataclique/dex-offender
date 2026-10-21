//! Ackee Bootcamp CTF — Level 5: Escrow Withdrawal.
//!
//! Exploit the escrow to withdraw funds.

use crate::solana::{SolChain, SolRoles};
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;

pub const PROGRAM_ID: &str = "5p55Gzc2FwGGHGp6DmncJaw9gbhLRpzj6Uca8Rv1bRhd";

#[derive(Debug, Clone)]
pub struct Target {
    pub program_id: Pubkey,
}

impl Target {
    pub fn set_up(
        chain: &Arc<dyn SolChain>,
        _roles: &SolRoles,
    ) -> eyre::Result<Self> {
        let program_id = PROGRAM_ID.parse::<Pubkey>()?;
        let so = "lib/solana-auditors-bootcamp/Capture-the-Flag/level-5/target/deploy/level_5.so";
        chain.deploy_program_from_file(program_id, so)?;
        Ok(Self { program_id })
    }

    pub fn check(&self, chain: &Arc<dyn SolChain>) -> bool {
        // Check that the escrow has been drained
        let _ = chain;
        false
    }
}
