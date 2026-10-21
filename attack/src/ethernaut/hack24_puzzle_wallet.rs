//! Solution scaffold for Ethernaut level 24 — puzzle_wallet.
//!
//! Propose + execute to drain proxy balance.

use async_trait::async_trait;
use ctf::Actor;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = ctf::ethernaut::lvl24_puzzle_wallet::Target;

    async fn attack(self, target: &Self::Target, offender: &Actor) -> eyre::Result<()> {
        // TODO: exploit the level
        let _ = (target, offender);
        Ok(())
    }
}
