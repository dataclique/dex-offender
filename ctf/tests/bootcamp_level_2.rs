//! Ackee bootcamp level 2 harness verification: set_up deploys the
//! program and the evaluation `check` reports unsolved (the explorers
//! cannot be initialized until the .so constraint issue documented in
//! the level module is resolved).

use ctf::solana::{LiteSvmChain, SolChain, SolRoles, bootcamp};
use std::sync::Arc;

#[test]
fn level_2_deploys_and_reports_unsolved() {
    let chain: Arc<dyn SolChain> = Arc::new(LiteSvmChain::new());
    let roles = SolRoles::new(chain.clone());

    let target = bootcamp::level_2::Target::set_up(&chain, &roles)
        .expect("level 2 set_up");

    assert!(
        !target.check(&chain),
        "level 2 check must be unsolved right after setup"
    );
}
