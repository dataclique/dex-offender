//! Ackee bootcamp level 3 harness verification: set_up deploys + founds
//! the faction (with the same-transaction mint top-up covering the
//! runtime's larger spl-token-2022 extension data); the evaluation
//! `check` reports unsolved right after setup.

use ctf::solana::{LiteSvmChain, SolChain, SolRoles, bootcamp};
use std::sync::Arc;

#[test]
fn level_3_setups_and_reports_unsolved() {
    let chain: Arc<dyn SolChain> = Arc::new(LiteSvmChain::new());
    let roles = SolRoles::new(chain.clone());

    let target = bootcamp::level_3::Target::set_up(&chain, &roles)
        .expect("level 3 set_up");

    assert!(
        !target.check(&chain),
        "level 3 check must be unsolved right after setup"
    );
}
