//! Template for new Ethernaut solution modules.
//!
//! Copy this file, rename it `hackNN_name.rs`, import the target from the
//! corresponding level module, and implement the `attack` method. Then
//! declare the module in `attack/src/ethernaut/mod.rs`.
//!
//! NOTE: hack01_fallback is the ONLY example solution. Do not add exploit
//! implementations to any other files — solve the challenges yourself!

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethernaut::lvlNN_name::Target;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(self, target: &Self::Target, offender: &Actor) -> eyre::Result<()> {
        // TODO: exploit the level
        let _ = (target, offender);
        Ok(())
    }
}
