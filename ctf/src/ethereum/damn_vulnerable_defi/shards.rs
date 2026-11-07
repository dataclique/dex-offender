//! Damn Vulnerable DeFi — Shards.
//!
//! Recover marketplace fee tokens to the designated recovery account in a
//! single transaction, preserving staking rewards.

use crate::{Level, Roles};
use alloy::{primitives::Address, primitives::U256, providers::Provider, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    DamnValuableToken,
    "../lib/damn-vulnerable-defi/out/DamnValuableToken.sol/DamnValuableToken.json"
);
sol!(
    #[sol(rpc)]
    DamnValuableNFT,
    "../lib/damn-vulnerable-defi/out/DamnValuableNFT.sol/DamnValuableNFT.json"
);
sol!(
    #[sol(rpc)]
    ShardsNFTMarketplace,
    "../lib/damn-vulnerable-defi/out/ShardsNFTMarketplace.sol/ShardsNFTMarketplace.json"
);
sol!(
    #[sol(rpc)]
    ShardsFeeVault,
    "../lib/damn-vulnerable-defi/out/ShardsFeeVault.sol/ShardsFeeVault.json"
);
sol!(
    #[sol(rpc)]
    DamnValuableStaking,
    "../lib/damn-vulnerable-defi/out/DamnValuableStaking.sol/DamnValuableStaking.json"
);

/// Upstream constants (test/shards/Shards.t.sol).
const STAKING_REWARDS: U256 = U256::from_limbs([
    100_000_000_000_000_000_000_000u128 as u64,
    (100_000_000_000_000_000_000_000u128 >> 64) as u64,
    0,
    0,
]);
const NFT_SUPPLY: u64 = 50;
const SELLER_NFT_BALANCE: u64 = 1;
const SELLER_DVT_BALANCE: U256 = U256::from_limbs([
    750_000_000_000_000_000_000u128 as u64,
    (750_000_000_000_000_000_000u128 >> 64) as u64,
    0,
    0,
]);
const STAKING_RATE: U256 =
    U256::from_limbs([1_000_000_000_000_000_000, 0, 0, 0]);
const MARKETPLACE_INITIAL_RATE: U256 =
    U256::from_limbs([75_000_000_000_000_000, 0, 0, 0]);
const NFT_OFFER_PRICE: u64 = 1_000_000_000_000;
const NFT_OFFER_SHARDS: U256 = U256::from_limbs([
    10_000_000_000_000_000_000_000_000u128 as u64,
    (10_000_000_000_000_000_000_000_000u128 >> 64) as u64,
    0,
    0,
]);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
    pub token: Address,
    pub staking: Address,
    /// The designated recovery account.
    pub recovery: Address,
    /// Marketplace fee tokens at setup (the recovery baseline).
    pub initial_tokens_in_marketplace: U256,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Shards"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethereum.damn_vulnerable_defi.shards.clone())
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let seller = &roles.some_user;
        let provider = roles.provider();

        println!("Deploying the Shards challenge...");

        // NFT supply: one NFT for the seller, the rest for the deployer.
        let nft = DamnValuableNFT::deploy(provider).await?;
        for id in 0..NFT_SUPPLY {
            let to = if id < SELLER_NFT_BALANCE {
                seller.address()
            } else {
                deployer.address()
            };
            nft.safeMint(to)
                .from(deployer.address())
                .send()
                .await?
                .watch()
                .await?;
        }

        let token = DamnValuableToken::deploy(provider).await?;
        let fee_vault_impl = ShardsFeeVault::deploy(provider).await?;
        let marketplace = ShardsNFTMarketplace::deploy(
            provider,
            *nft.address(),
            *token.address(),
            *fee_vault_impl.address(),
            deployer.address(),
            MARKETPLACE_INITIAL_RATE,
        )
        .await?;

        // Staking contract holds the rewards; the fee vault stakes its fees.
        let staking = DamnValuableStaking::deploy(
            provider,
            *token.address(),
            STAKING_RATE,
        )
        .await?;
        token
            .transfer(*staking.address(), STAKING_REWARDS)
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        let fee_vault = ShardsFeeVault::new(
            marketplace.feeVault().call().await?,
            provider.clone(),
        );
        fee_vault
            .enableStaking(*staking.address())
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        // The seller funds fees and opens the single upstream offer.
        token
            .transfer(seller.address(), SELLER_DVT_BALANCE)
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        token
            .approve(*marketplace.address(), SELLER_DVT_BALANCE)
            .from(seller.address())
            .send()
            .await?
            .watch()
            .await?;
        nft.setApprovalForAll(*marketplace.address(), true)
            .from(seller.address())
            .send()
            .await?
            .watch()
            .await?;
        marketplace
            .openOffer(
                U256::ZERO,
                NFT_OFFER_SHARDS,
                U256::from(NFT_OFFER_PRICE),
            )
            .from(seller.address())
            .send()
            .await?
            .watch()
            .await?;

        let initial_tokens_in_marketplace =
            token.balanceOf(*marketplace.address()).call().await?;
        eyre::ensure!(
            marketplace.offerCount().call().await? == SELLER_NFT_BALANCE,
            "seller offer missing"
        );
        eyre::ensure!(
            initial_tokens_in_marketplace > U256::ZERO,
            "marketplace holds no fee escrow"
        );

        Ok(Target {
            address: *marketplace.address(),
            token: *token.address(),
            staking: *staking.address(),
            recovery: recovery_account(),
            initial_tokens_in_marketplace,
        })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let offender = &roles.offender;
        let provider = roles.provider();
        let token = DamnValuableToken::new(self.token, provider.clone());

        println!(
            "Checking that marketplace fees were recovered in one transaction..."
        );
        let staking_balance = token.balanceOf(self.staking).call().await?;
        let marketplace_balance = token.balanceOf(self.address).call().await?;
        let missing = self
            .initial_tokens_in_marketplace
            .saturating_sub(marketplace_balance);

        // Staking rewards untouched; more than 1% of fees recovered; the
        // recovery account holds exactly what left the marketplace; the
        // player keeps nothing and used a single transaction.
        Ok(staking_balance == STAKING_REWARDS
            && missing
                > self.initial_tokens_in_marketplace / U256::from(100u64)
            && token.balanceOf(self.recovery).call().await? == missing
            && token.balanceOf(offender.address()).call().await? == U256::ZERO
            && provider.get_transaction_count(offender.address()).await? == 1)
    }
}

/// The upstream `recovery` account, mapped to a fixed local address (anvil
/// account #4 — well-known dev key, never used by the harness).
pub fn recovery_account() -> Address {
    Address::new([
        0x90, 0xF7, 0x9b, 0xf6, 0xEB, 0x2c, 0x4f, 0x87, 0x03, 0x65, 0xE7, 0x85,
        0x98, 0x2E, 0x1f, 0x10, 0x1E, 0x93, 0xb9, 0x06,
    ])
}
