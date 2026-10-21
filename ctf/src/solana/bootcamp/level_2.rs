//! Ackee Bootcamp CTF — Level 2: Monster Battle.
//!
//! Battle monsters, heal allies, and reveal the secret.

use crate::solana::{SolChain, SolRoles};
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;

pub const PROGRAM_ID: &str = "5pFB5g5WvBcL5fkaLY5Nz9cqEAxnkRPVgc3DCTKFY7uK";

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
        let so = "lib/solana-auditors-bootcamp/Capture-the-Flag/level-2/target/deploy/level_2.so";
        chain.deploy_program_from_file(program_id, so)?;
        Ok(Self { program_id })
    }

    pub fn check(&self, chain: &Arc<dyn SolChain>) -> bool {
        // Check that the secret has been revealed
        // TODO: determine the exact PDA and state check from the program source
        let _ = chain;
        false
    }
}
