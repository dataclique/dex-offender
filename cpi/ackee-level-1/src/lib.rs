//! Generated interface for the Bootcamp Level 1 program.
//!
//! Anchor generates crate-root references and uses Solana 2 types. Keep it in
//! its own crate so those types do not collide with the runner's Solana SDK.

anchor_gen::generate_cpi_interface!(
    idl_path = "../../lib/solana-auditors-bootcamp/Capture-the-Flag/level-1/target/idl/level_1.json"
);
