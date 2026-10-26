//! mysten-ctf sabotage_arena solution — practice stub (no answer).
//!
//! The arena mints 10 flags; `claim_flag` requires a registered
//! player's shield ≥ 12 despite the 600s build cooldown. The exploit
//! would reach the threshold — intentionally not implemented.
#![allow(unused)]

use ctf::sui::mysten_ctf::sabotage_arena::Target;

pub async fn attack(
    _target: &Target,
    _roles: &ctf::sui::SuiRoles,
    _localnet: &ctf::sui::SuiLocalnet,
) -> eyre::Result<()> {
    todo!("practice stub — build shield ≥ 12 within cooldown, then claim")
}
