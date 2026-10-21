//! Ackee Bootcamp CTF — Level 1: Ancient Vault.
//!
//! Access the ancient vault by calling the `access_vault` instruction.

use crate::solana::{SolChain, SolRoles};
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;

pub const PROGRAM_ID: &str = "7u6UPvrh9stwx6ApoYPyvn62ScvQtUc3fJgMQXKvbViK";

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
        let so = "lib/solana-auditors-bootcamp/Capture-the-Flag/level-1/target/deploy/level_1.so";
        chain.deploy_program_from_file(program_id, so)?;
        Ok(Self { program_id })
    }

    /// PDA for the AncientVault account.
    pub fn vault_pda(&self) -> Pubkey {
        Pubkey::find_program_address(&[b"ancient_vault"], &self.program_id).0
    }

    pub fn check(&self, chain: &Arc<dyn SolChain>) -> bool {
        // Check that the vault account has been accessed (state changed)
        let vault = self.vault_pda();
        chain.get_account(&vault).is_some()
    }
}
