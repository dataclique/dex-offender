//! Sui-chain solution modules (practice stubs only).
//!
//! Attack callbacks receive only their offender actor, never setup roles.
//!
//! ```compile_fail
//! async fn cannot_pass_setup_roles(
//!     target: &ctf::sui::lets_ctf::check_in::Target,
//!     roles: &ctf::sui::SuiRoles,
//!     localnet: &ctf::sui::SuiLocalnet,
//! ) -> eyre::Result<()> {
//!     attack::sui::lets_ctf::hack_check_in::attack(target, roles, localnet).await
//! }
//! ```
//!
//! ```compile_fail
//! async fn cannot_pass_setup_roles(
//!     target: &ctf::sui::mysten_ctf::moving_window::Target,
//!     roles: &ctf::sui::SuiRoles,
//!     localnet: &ctf::sui::SuiLocalnet,
//! ) -> eyre::Result<()> {
//!     attack::sui::mysten_ctf::hack_moving_window::attack(target, roles, localnet).await
//! }
//! ```
//!
//! ```compile_fail
//! async fn cannot_pass_setup_roles(
//!     target: &ctf::sui::mysten_ctf::sabotage_arena::Target,
//!     roles: &ctf::sui::SuiRoles,
//!     localnet: &ctf::sui::SuiLocalnet,
//! ) -> eyre::Result<()> {
//!     attack::sui::mysten_ctf::hack_sabotage_arena::attack(target, roles, localnet).await
//! }
//! ```
//!
//! ```compile_fail
//! async fn cannot_pass_setup_roles(
//!     target: &ctf::sui::mysten_ctf::lootboxes::Target,
//!     roles: &ctf::sui::SuiRoles,
//!     localnet: &ctf::sui::SuiLocalnet,
//! ) -> eyre::Result<()> {
//!     attack::sui::mysten_ctf::hack_lootboxes::attack(target, roles, localnet).await
//! }
//! ```
//!
#![allow(unused)]

pub mod lets_ctf;
pub mod mysten_ctf;
