//! hana challenge-one harness verification.
//!
//! Threat-model-first boundary notes for this module:
//! - The prebuilt `challenge1.so` runs inside in-process LiteSVM (no
//!   network, ephemeral state, dev keypairs only) — the challenge itself
//!   is the untrusted-by-design payload, contained by the sandbox.
//! - The one deserialization boundary is `check()`: raw spl-token
//!   account bytes from the chain. The abuse cases below pin that a
//!   missing or short account NEVER panics — it must read as unsolved.

use ctf::solana::{LiteSvmChain, SolChain, SolRoles, hana};
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;

#[test]
fn hana_check_missing_account_is_unsolved_not_panic() {
    let chain: Arc<dyn SolChain> = Arc::new(LiteSvmChain::new());
    let target = hana::Target {
        program_id: Pubkey::new_unique(),
        deposit_account: Pubkey::new_unique(), // never created
    };
    assert!(
        !target.check(&chain),
        "missing deposit account must read unsolved, not panic"
    );
}

#[test]
fn hana_check_short_account_data_is_unsolved_not_panic() {
    let chain: Arc<dyn SolChain> = Arc::new(LiteSvmChain::new());
    let program = Pubkey::new_unique();
    let deposit = Pubkey::new_unique();
    // Adversarial short buffer at the deserialization boundary.
    chain.set_account(deposit, vec![0u8; 8], program);
    let target = hana::Target { program_id: program, deposit_account: deposit };
    assert!(
        !target.check(&chain),
        "short account data must read unsolved, not panic"
    );
}

#[test]
fn hana_setup_deploys_and_reports_unsolved() {
    let chain: Arc<dyn SolChain> =
        Arc::new(LiteSvmChain::with_legacy_programs());
    let roles = SolRoles::new(chain.clone());

    let target = hana::Target::set_up(&chain, &roles).expect("hana set_up");

    assert!(
        !target.check(&chain),
        "fresh setup must report unsolved (deposit holds 1 bitcoin)"
    );
}
