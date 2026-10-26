//! lets-ctf check_in solution — practice stub (no answer).
//!
//! The level shares a `FlagString` whose `ture_num` increments on each
//! valid sha3 preimage submission. The exploit would compute the
//! preimage of the current string — intentionally not implemented.
#![allow(unused)]

use ctf::sui::lets_ctf::check_in::Target;

pub async fn attack(
    _target: &Target,
    _offender: &ctf::sui::SuiRoles,
    _localnet: &ctf::sui::SuiLocalnet,
) -> eyre::Result<()> {
    todo!("practice stub — compute the sha3 preimage and call get_flag")
}
