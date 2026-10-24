//! Ackee bootcamp level 5 harness verification: set_up deploys both
//! programs + builds the guardian escrow; the evaluation `check`
//! reports unsolved right after setup.

use ctf::solana::{LiteSvmChain, SolChain, SolRoles, bootcamp};
use std::sync::Arc;

#[test]
fn level_5_setups_and_reports_unsolved() {
    let chain: Arc<dyn SolChain> = Arc::new(LiteSvmChain::new());
    let roles = SolRoles::new(chain.clone());

    let target = bootcamp::level_5::Target::set_up(&chain, &roles)
        .expect("level 5 set_up");

    assert!(
        !target.check(&chain),
        "level 5 check must be unsolved right after setup"
    );
}
