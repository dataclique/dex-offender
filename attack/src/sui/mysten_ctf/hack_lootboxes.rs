//! mysten-ctf lootboxes solution — practice stub (no answer).
//!
//! `open_lootbox` burns a 12 USDC payment and randomly mints a
//! `MaybeFlag`. How to obtain and keep a real Flag is the level's
//! answer — intentionally not implemented.
#![allow(unused)]

use ctf::sui::mysten_ctf::lootboxes::Target;

pub async fn attack(
    _target: &Target,
    _roles: &ctf::sui::SuiRoles,
    _localnet: &ctf::sui::SuiLocalnet,
) -> eyre::Result<()> {
    todo!("practice stub — open boxes until the flag appears")
}
