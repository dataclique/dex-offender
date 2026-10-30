//! Ethernaut 36 — Cashback.
//!
//! Goal: Reach the maximum native-currency and FREE cashback balances in
//! `Cashback`, own at least two Super Cashback NFTs including your
//! address-derived NFT, and leave your account delegated to the Cashback
//! instance as required by its factory check.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Cashback.sol

use crate::{Level, Roles, to_ether};
use alloy::{
    primitives::{Address, U256, address},
    providers::Provider,
    sol,
};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    Cashback,
    "../lib/ethernaut/contracts/out/Cashback.sol/Cashback.json"
);

sol!(
    #[sol(rpc)]
    FreedomCoin,
    "../lib/ethernaut/contracts/out/CashbackFactory.sol/FreedomCoin.json"
);

sol!(
    #[sol(rpc)]
    SuperCashbackNFT,
    "../lib/ethernaut/contracts/out/CashbackFactory.sol/SuperCashbackNFT.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
    /// The FreedomCoin currency address (not exposed by the Cashback ABI).
    pub free: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Cashback"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let t = &ctfs.ethereum.ethernaut.level36;
        Ok(Target { address: t.address, free: t.free })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        // Mirror CashbackFactory: FreedomCoin (BOB holds 100 ether),
        // SuperCashbackNFT, then the Cashback escrow with native + FREE
        // currencies and their rates/max caps.
        let free = FreedomCoin::deploy(provider).await?;
        free.mint(
            address!("0000000000000000000000000000000000000B0B"),
            to_ether(100),
        )
        .from(deployer.address())
        .send()
        .await?
        .watch()
        .await?;

        let nft = SuperCashbackNFT::deploy(provider).await?;
        let native_currency =
            address!("EeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE");
        let contract = Cashback::deploy(
            provider,
            vec![native_currency, *free.address()],
            vec![U256::from(50), U256::from(200)],
            vec![to_ether(1), to_ether(500)],
            *nft.address(),
        )
        .await?;

        nft.transferOwnership(*contract.address())
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        println!("Deployed the Cashback at {}", contract.address());
        Ok(Target { address: *contract.address(), free: *free.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let offender = roles.offender.address();
        let provider = roles.provider();
        let contract = Cashback::new(self.address, provider.clone());

        // Mirror validateInstance: maxed native + FREE cashback, the
        // player-owned Super Cashback NFT for their own address id, a
        // second NFT, and a 23-byte ef0100-delegation code at the player.
        let native_currency =
            address!("EeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE");
        let free_addr = self.free;
        // ERC-1155-style id for a currency is uint160(currency).
        let id = |a: Address| U256::from_be_bytes(a.into_word().0);
        let native =
            contract.balanceOf(offender, id(native_currency)).call().await?;
        let free = contract.balanceOf(offender, id(free_addr)).call().await?;
        // Cheap non-reverting conditions first: an unsolved player holds no
        // maxed cashback, so the NFT queries below usually never run.
        if native != to_ether(1) || free != to_ether(500) {
            return Ok(false);
        }
        let nft = SuperCashbackNFT::new(
            contract.superCashbackNFT().call().await?,
            provider.clone(),
        );
        // ownerOf reverts with ERC721NonexistentToken until the NFT exists;
        // that exact revert means unsolved — anything else is a real error.
        let nonexistent_token_selector =
            &alloy::primitives::keccak256(b"ERC721NonexistentToken(uint256)").0
                [..4];
        let owner = match nft
            .ownerOf(U256::from_be_bytes(offender.into_word().0))
            .call()
            .await
        {
            Ok(owner) => owner,
            Err(error) => {
                let revert_data = error
                    .as_revert_data()
                    .ok_or_else(|| eyre::eyre!("ownerOf failed: {error:#}"))?;
                if revert_data.get(0..4) == Some(nonexistent_token_selector) {
                    return Ok(false);
                }
                return Err(eyre::eyre!("ownerOf failed: {error:#}"));
            }
        };
        let nft_balance = nft.balanceOf(offender).call().await?;

        let code = provider.get_code_at(offender).await?;
        let mut expected = [0u8; 23];
        expected[0] = 0xef;
        expected[1] = 0x01;
        expected[2] = 0x00;
        expected[3..].copy_from_slice(self.address.as_slice());

        Ok(native == to_ether(1)
            && free == to_ether(500)
            && owner == offender
            && nft_balance >= U256::from(2)
            && code.as_ref() == expected)
    }
}
