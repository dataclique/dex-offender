//! Solana solution scaffolds, one directory per wargame (mirroring the
//! EVM layout under `attack/src/ethereum/`).
//!
//! All five Ackee levels are enabled in the runner. Start Surfpool and
//! deploy all levels as described in the repository README.
//! Attack callbacks receive only the offender's `SolActor`, which implements
//! Solana's native `Signer` trait and exposes `rpc_client() -> &RpcClient`.
//! Build transactions with the Solana SDK and use that client directly.
//! Setup identities that the levels require you to co-sign with (level
//! 2's two explorer users, level 3's faction authority) are exposed by
//! the level's `Target`.
//!
//! ## Role boundary
//!
//! Attack callbacks cannot receive setup roles.
//!
//! ```compile_fail
//! async fn cannot_pass_setup_roles(
//!     target: &ctf::solana::ackee::level_1::Target,
//!     roles: &ctf::solana::SolRoles,
//! ) -> eyre::Result<()> {
//!     attack::solana::ackee::hack1_ancient_vault::attack(target, roles).await
//! }
//! ```
//!
//! ```compile_fail
//! async fn cannot_pass_setup_roles(
//!     target: &ctf::solana::ackee::level_2::Target,
//!     roles: &ctf::solana::SolRoles,
//! ) -> eyre::Result<()> {
//!     attack::solana::ackee::hack2_monster_battle::attack(target, roles).await
//! }
//! ```
//!
//! ```compile_fail
//! async fn cannot_pass_setup_roles(
//!     target: &ctf::solana::ackee::level_3::Target,
//!     roles: &ctf::solana::SolRoles,
//! ) -> eyre::Result<()> {
//!     attack::solana::ackee::hack3_hidden_faction::attack(target, roles).await
//! }
//! ```
//!
//! ```compile_fail
//! async fn cannot_pass_setup_roles(
//!     target: &ctf::solana::ackee::level_4::Target,
//!     roles: &ctf::solana::SolRoles,
//! ) -> eyre::Result<()> {
//!     attack::solana::ackee::hack4_vesting::attack(target, roles).await
//! }
//! ```
//!
//! ```compile_fail
//! async fn cannot_pass_setup_roles(
//!     target: &ctf::solana::ackee::level_5::Target,
//!     roles: &ctf::solana::SolRoles,
//! ) -> eyre::Result<()> {
//!     attack::solana::ackee::hack5_guardian_escrow::attack(target, roles).await
//! }
//! ```
//!
//! ```compile_fail
//! async fn cannot_pass_setup_roles(
//!     target: &ctf::solana::hana::Target,
//!     roles: &ctf::solana::SolRoles,
//! ) -> eyre::Result<()> {
//!     attack::solana::paradigm2022::hack_hana::attack(target, roles).await
//! }
//! ```
//!

pub mod ackee;
pub mod paradigm2022;

/// The player's discovered secret for a secret-reveal level (3-5),
/// saved as plain text under `.solana/answers/ackee-level-{level}.txt`.
/// The runner submits it to the level's program, which alone judges
/// whether it is the real secret.
pub fn player_answer(level: u8) -> Option<String> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(format!("../.solana/answers/ackee-level-{level}.txt"));
    std::fs::read_to_string(path)
        .ok()
        .map(|text| text.trim_end_matches(['\r', '\n']).to_string())
        .filter(|secret| !secret.is_empty())
}

#[cfg(test)]
mod practice {
    use crate::solana::ackee::hack1_ancient_vault;
    use ctf::solana::{SolActor, ackee::level_1, rpc};

    #[tokio::test]
    #[ignore = "requires the deployed local Surfpool described in README.md"]
    async fn solana_progression() -> eyre::Result<()> {
        let offender =
            SolActor::from_file(&rpc::rpc_url()?, rpc::keypair_path())?;
        let target = level_1::Target::from_rpc(offender.rpc_client()).await?;
        if !target.check_rpc(offender.rpc_client()).await? {
            hack1_ancient_vault::attack(&target, &offender).await?;
        }
        eyre::ensure!(
            target.check_rpc(offender.rpc_client()).await?,
            "Ancient Vault is not solved"
        );
        Ok(())
    }
}
