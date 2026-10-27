//! Ethernaut 33 — Magic Animal Carousel.
//!
//! Goal: Break `MagicAnimalCarousel`'s animal-storage rule: after the checker
//! stores "Goat" and spins, the current crate must not contain the expected
//! animal encoding.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/MagicAnimalCarousel.sol

use async_trait::async_trait;
use ctf::ethereum::ethernaut::lvl33_magic_animal_carousel::*;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        // TODO: exploit the level.
        let _ = (target, offender);
        Ok(())
    }
}
