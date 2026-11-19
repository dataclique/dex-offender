//! Ackee Bootcamp CTF — Level 2: Monster Battle.
//!
//! `reveal_secret` must succeed for both explorers. Each needs enough
//! experience and monsters defeated, and the two supplied secrets must
//! match — discovering values the program accepts is part of the
//! exploit, and no answer material lives here. The explorers start
//! uninitialized.
//!
//! Setup deploys the program and funds the two persistent user
//! identities; their `EXPLORER` PDAs derive from the program and user
//! addresses. Evaluation is the upstream `reveal_secret` transaction
//! signed by both users: it succeeds only from the state the exploit
//! must reach.

use crate::solana::{ackee::Keyfile, rpc};
use serde::{Deserialize, Serialize};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
};
use std::sync::Arc;

pub const PROGRAM_ID: &str = "5pFB5g5WvBcL5fkaLY5Nz9cqEAxnkRPVgc3DCTKFY7uK";

const SO: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../lib/solana-auditors-bootcamp/Capture-the-Flag/level-2/target/deploy/level_2.so"
);

/// Anchor 8-byte instruction discriminator: sha256("global:<name>")[..8].
fn discriminator(name: &str) -> [u8; 8] {
    let hash = solana_sdk::hash::hash(format!("global:{name}").as_bytes());
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&hash.as_ref()[..8]);
    discriminator
}

fn explorer_pda(program: &Pubkey, user: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"EXPLORER", user.as_ref()], program).0
}

/// The two persistent explorer users. `reveal_secret` requires both
/// signatures, so their identities are created once and persisted.
#[derive(Serialize, Deserialize)]
pub struct Actors {
    /// First explorer: pays for its initialization and the evaluation
    /// transaction.
    pub user1: Keyfile,
    /// Second explorer: signs alongside `user1`.
    pub user2: Keyfile,
}

impl Actors {
    pub(super) fn generate() -> Self {
        Self { user1: Keyfile::generate(), user2: Keyfile::generate() }
    }

    fn users(&self) -> eyre::Result<[Arc<Keypair>; 2]> {
        Ok([Arc::new(self.user1.keypair()?), Arc::new(self.user2.keypair()?)])
    }
}

#[derive(Clone)]
pub struct Target {
    pub program_id: Pubkey,
    /// First persistent explorer user (see [`Actors::user1`]).
    pub user1: Arc<Keypair>,
    /// Second persistent explorer user (see [`Actors::user2`]).
    pub user2: Arc<Keypair>,
    /// First explorer's account PDA, derived from `user1`.
    pub explorer1: Pubkey,
    /// Second explorer's account PDA, derived from `user2`.
    pub explorer2: Pubkey,
}

impl Target {
    /// Deploy the program and fund both users on the persistent chain.
    pub async fn set_up(
        rpc: &RpcClient,
        actors: &Actors,
        _offender: &Keypair,
    ) -> eyre::Result<()> {
        rpc::write_program_if_absent(rpc, PROGRAM_ID, SO).await?;
        let [user1, user2] = actors.users()?;
        rpc::fund_at_least(rpc, &user1.pubkey(), 1_000_000_000).await?;
        rpc::fund_at_least(rpc, &user2.pubkey(), 1_000_000_000).await?;
        Ok(())
    }

    /// Load the level from the persistent chain.
    pub async fn from_rpc(
        rpc: &RpcClient,
        actors: &Actors,
    ) -> eyre::Result<Self> {
        let program_id =
            super::require_program(rpc, PROGRAM_ID, "level 2 (Monster Battle)")
                .await?;
        let [user1, user2] = actors.users()?;
        Ok(Self {
            program_id,
            explorer1: explorer_pda(&program_id, &user1.pubkey()),
            explorer2: explorer_pda(&program_id, &user2.pubkey()),
            user1,
            user2,
        })
    }

    /// Evaluation: the upstream `reveal_secret` transaction signed by
    /// both users. It succeeds only when both explorers reached the
    /// state the challenge requires; any failure means unsolved.
    pub async fn check_rpc(&self, rpc: &RpcClient) -> eyre::Result<bool> {
        let instruction = Instruction::new_with_bytes(
            self.program_id,
            &discriminator("reveal_secret"),
            vec![
                AccountMeta::new(self.user1.pubkey(), true),
                AccountMeta::new(self.user2.pubkey(), true),
                AccountMeta::new(self.explorer1, false),
                AccountMeta::new(self.explorer2, false),
            ],
        );
        Ok(rpc::send_instructions(
            rpc,
            &self.user1,
            &[instruction],
            &[&self.user2],
        )
        .await
        .is_ok())
    }
}
