//! Generated interface for the Bootcamp Level 1 program.
//!
//! Anchor generates crate-root references and uses Solana 2 types. Keep it in
//! its own crate so those types do not collide with the runner's Solana SDK.

// The IDL's AccountInfo-typed accounts make anchor_gen emit calls to
// anchor_lang::deprecated_account_info_usage. The interface mirrors the
// pinned upstream IDL, so the generated deprecation is accepted here.
#![allow(deprecated)]

anchor_gen::generate_cpi_interface!(
    idl_path = "../../lib/solana-auditors-bootcamp/Capture-the-Flag/level-1/target/idl/level_1.json"
);
