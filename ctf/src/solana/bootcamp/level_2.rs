//! Ackee Bootcamp CTF — Level 2: Monster Battle.
//!
//! Battle monsters, heal allies, and reveal the secret. The evaluation
//! mirrors the upstream test: `reveal_secret` succeeds only when both
//! explorers have enough experience and monsters defeated (and matching
//! secrets, which the exploit must arrange).
//!
//! SETUP STATUS: program deployment works; `init_explorer` currently
//! fails inside the .so with `InvalidProgramId` on its `system_program`
//! account constraint (Left `4vJ9JU1bJJE96FWSJKvHsmmFADCg4gpZQff4P3bkLKi`,
//! Right `1111…1`) regardless of which pubkey we pass in that slot —
//! the binary's account-matching does not line up with the shipped IDL.
//! Needs .so-level disassembly (or a rebuilt program) before the two
//! explorers can be initialized; `check` stays unsolved until then.

use crate::solana::SolChain;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use std::sync::Arc;

pub const PROGRAM_ID: &str = "5pFB5g5WvBcL5fkaLY5Nz9cqEAxnkRPVgc3DCTKFY7uK";

#[derive(Clone)]
pub struct Target {
    pub program_id: Pubkey,
    pub user1: Arc<Keypair>,
    pub user2: Arc<Keypair>,
    pub explorer1: Pubkey,
    pub explorer2: Pubkey,
}

/// Anchor 8-byte discriminator: sha256("global:<name>")[..8].
fn discriminator(name: &str) -> [u8; 8] {
    let h = solana_sdk::hash::hash(format!("global:{name}").as_bytes());
    let mut d = [0u8; 8];
    d.copy_from_slice(&h.as_ref()[..8]);
    d
}

fn explorer_pda(program: &Pubkey, user: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"EXPLORER", user.as_ref()], program).0
}

impl Target {
    pub fn set_up(
        chain: &Arc<dyn SolChain>,
        _roles: &crate::solana::SolRoles,
    ) -> eyre::Result<Self> {
        let program_id = PROGRAM_ID.parse::<Pubkey>()?;
        // Workspace-root-relative, resolved at compile time so the level
        // works whether tests run from the member dir or the root.
        let so = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../lib/solana-auditors-bootcamp/Capture-the-Flag/level-2/target/deploy/level_2.so"
        );
        chain.deploy_program_from_file(program_id, so)?;

        // Explorer initialization is blocked on the .so constraint issue
        // above; the evaluation accounts are still derived and exposed so
        // the check is ready once init works.
        let user1 = Arc::new(Keypair::new());
        let user2 = Arc::new(Keypair::new());
        let explorer1 = explorer_pda(&program_id, &user1.pubkey());
        let explorer2 = explorer_pda(&program_id, &user2.pubkey());

        Ok(Self { program_id, user1, user2, explorer1, explorer2 })
    }

    /// Evaluation: `reveal_secret` succeeds only when the challenge is
    /// solved. Without initialized explorers it cannot succeed, so this
    /// correctly reports unsolved until the init flow is fixed.
    pub fn check(&self, chain: &Arc<dyn SolChain>) -> bool {
        let ix = Instruction::new_with_bytes(
            self.program_id,
            &discriminator("reveal_secret"),
            vec![
                AccountMeta::new(self.user1.pubkey(), true),
                AccountMeta::new(self.user2.pubkey(), true),
                AccountMeta::new(self.explorer1, false),
                AccountMeta::new(self.explorer2, false),
            ],
        );
        chain.send_signed(&[&self.user1, &self.user2], &[ix]).is_ok()
    }
}
