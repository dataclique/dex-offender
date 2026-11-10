//! Solana solution scaffolds, one directory per wargame (mirroring the
//! EVM layout under `attack/src/ethereum/`).
//!
//! Only Ackee Bootcamp 1 is enabled in the runner. Start Surfpool and
//! deploy the first level as described in the repository README.
//! Attack callbacks receive only the offender's `SolActor`, which implements
//! Solana's native `Signer` trait and exposes `rpc_client() -> &RpcClient`.
//! Build transactions with the Solana SDK and use that client directly.
//! Later level scaffolds remain available but are not run or deployed.
//!
//! ## Role boundary
//!
//! Attack callbacks cannot receive setup roles.
//!
//! ```compile_fail
//! async fn cannot_pass_setup_roles(
//!     target: &ctf::solana::bootcamp::level_1::Target,
//!     roles: &ctf::solana::SolRoles,
//! ) -> eyre::Result<()> {
//!     attack::solana::ackee::hack1_ancient_vault::attack(target, roles).await
//! }
//! ```
//!
//! ```compile_fail
//! async fn cannot_pass_setup_roles(
//!     target: &ctf::solana::bootcamp::level_2::Target,
//!     roles: &ctf::solana::SolRoles,
//! ) -> eyre::Result<()> {
//!     attack::solana::ackee::hack2_monster_battle::attack(target, roles).await
//! }
//! ```
//!
//! ```compile_fail
//! async fn cannot_pass_setup_roles(
//!     target: &ctf::solana::bootcamp::level_3::Target,
//!     roles: &ctf::solana::SolRoles,
//! ) -> eyre::Result<()> {
//!     attack::solana::ackee::hack3_hidden_faction::attack(target, roles).await
//! }
//! ```
//!
//! ```compile_fail
//! async fn cannot_pass_setup_roles(
//!     target: &ctf::solana::bootcamp::level_4::Target,
//!     roles: &ctf::solana::SolRoles,
//! ) -> eyre::Result<()> {
//!     attack::solana::ackee::hack4_vesting::attack(target, roles).await
//! }
//! ```
//!
//! ```compile_fail
//! async fn cannot_pass_setup_roles(
//!     target: &ctf::solana::bootcamp::level_5::Target,
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

#[cfg(test)]
mod practice {
    use crate::solana::ackee::hack1_ancient_vault;
    use ctf::solana::{SolActor, bootcamp::level_1, rpc};

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
