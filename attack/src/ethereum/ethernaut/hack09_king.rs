//! Ethernaut 09 — King.
//!
//! Goal: Claim the throne in `King` and prevent the original owner from
//! reclaiming it during the level check.
//!
//
// Upstream hint:
//
// The contract below represents a very simple game: whoever sends it an
// amount of ether that is larger than the current prize becomes the new
// king. On such an event, the overthrown king gets paid the new prize,
// making a bit of ether in the process! As ponzi as it gets xD
//
// Such a fun game. Your goal is to break it.
//
// When you submit the instance back to the level, the level is going to
// reclaim kingship. You will beat the level if you can avoid such a self
// proclamation.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/King.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl09_king::*;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        // TODO: exploit the level
        let _ = (target, offender);
        Ok(())
    }
}
