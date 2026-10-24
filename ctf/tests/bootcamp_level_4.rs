//! Ackee bootcamp level 4 harness verification: set_up deploys + builds
//! the vesting escrow; the evaluation `check` reports unsolved right
//! after setup (wrong secret).

use ctf::solana::{LiteSvmChain, SolChain, SolRoles, bootcamp};
use std::sync::Arc;

#[test]
fn level_4_setups_and_reports_unsolved() {
    let chain: Arc<dyn SolChain> = Arc::new(LiteSvmChain::new());
    let roles = SolRoles::new(chain.clone());

    let target = bootcamp::level_4::Target::set_up(&chain, &roles)
        .expect("level 4 set_up");

    assert!(
        !target.check(&chain),
        "level 4 check must be unsolved right after setup"
    );
}
