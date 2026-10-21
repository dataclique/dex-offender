//! Ackee Solana Auditors Bootcamp CTF levels 1–5 on the alloy Solana core.

pub mod level_1;
pub mod level_2;
pub mod level_3;
pub mod level_4;
pub mod level_5;

use crate::solana::{SolActor, SolChain, SolRoles};
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;

/// Deploy a bootcamp level program from its prebuilt .so and return the
/// program ID.
fn deploy_level(
    chain: &Arc<dyn SolChain>,
    level: u8,
    program_id: &str,
) -> eyre::Result<Pubkey> {
    let id = program_id.parse::<Pubkey>()?;
    let so_path = format!(
        "lib/solana-auditors-bootcamp/Capture-the-Flag/level-{level}/target/deploy/level_{level}.so"
    );
    chain.deploy_program_from_file(id, &so_path)?;
    Ok(id)
}

pub struct AckeeBootcamp {
    pub level_1: Pubkey,
    pub level_2: Pubkey,
    pub level_3: Pubkey,
    pub level_4: Pubkey,
    pub level_5: Pubkey,
}

pub const LEVEL_1_ID: &str = "7u6UPvrh9stwx6ApoYPyvn62ScvQtUc3fJgMQXKvbViK";
pub const LEVEL_2_ID: &str = "5pFB5g5WvBcL5fkaLY5Nz9cqEAxnkRPVgc3DCTKFY7uK";
pub const LEVEL_3_ID: &str = "5zjbNpnsSkCNG6zHzK183ujm6dn6fWeHWeUnk1Rzrs1Y";
pub const LEVEL_4_ID: &str = "D51vhx6jAbBtQVwo1fcYr7RMKQKAAnSUy6v7vRCHCZL3";
pub const LEVEL_5_ID: &str = "5p55Gzc2FwGGHGp6DmncJaw9gbhLRpzj6Uca8Rv1bRhd";

pub fn set_up_ackee(
    chain: &Arc<dyn SolChain>,
    _roles: &SolRoles,
) -> eyre::Result<AckeeBootcamp> {
    println!("Deploying Ackee Bootcamp CTF levels...");
    let level_1 = deploy_level(chain, 1, LEVEL_1_ID)?;
    let level_2 = deploy_level(chain, 2, LEVEL_2_ID)?;
    let level_3 = deploy_level(chain, 3, LEVEL_3_ID)?;
    let level_4 = deploy_level(chain, 4, LEVEL_4_ID)?;
    let level_5 = deploy_level(chain, 5, LEVEL_5_ID)?;
    Ok(AckeeBootcamp { level_1, level_2, level_3, level_4, level_5 })
}
