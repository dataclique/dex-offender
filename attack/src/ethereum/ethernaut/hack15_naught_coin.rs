//! Ethernaut 15 — Naught Coin.
//!
//! Goal: Move all of your offender account's `NaughtCoin` tokens away despite
//! the ten-year transfer lock. Its final token balance must be zero.
//!
//
// Upstream hint:
//
// NaughtCoin is an ERC20 token and you're already holding all of them. The
// catch is that you'll only be able to transfer them after a 10 year lockout
// period. Can you figure out how to get them out to another address so that
// you can transfer them freely? Complete this level by getting your token
// balance to 0.
//
// Things that might help
// * The [ERC20](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-20.md)
// Spec
// * The
// [OpenZeppelin](https://github.com/OpenZeppelin/zeppelin-solidity/tree/master/contracts)
// codebase
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/NaughtCoin.sol

use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl15_naught_coin::*;

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
