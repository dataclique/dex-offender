//! Ackee Bootcamp CTF — Level 3: Faction Secret.
//!
//! Obtain the faction token and reveal the faction secret.

use crate::solana::{SolChain, SolRoles};
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;

pub const PROGRAM_ID: &str = "5zjbNpnsSkCNG6zHzK183ujm6dn6fWeHWeUnk1Rzrs1Y";

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
        let so = "lib/solana-auditors-bootcamp/Capture-the-Flag/level-3/target/deploy/level_3.so";
        chain.deploy_program_from_file(program_id, so)?;
        Ok(Self { program_id })
    }

    pub fn check(&self, chain: &Arc<dyn SolChain>) -> bool {
        // Check that the faction secret has been revealed
        let _ = chain;
        false
    }
}
