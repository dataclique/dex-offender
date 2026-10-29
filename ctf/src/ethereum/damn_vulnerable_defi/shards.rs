//! Damn Vulnerable DeFi — Shards.
//!
//! Buy the Damn Valuable NFT for 1% of its price.

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, primitives::U256, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    DamnValuableToken,
    "abi/damn-vulnerable-defi/DamnValuableToken.json"
);
sol!(
    #[sol(rpc)]
    DamnValuableNFT,
    "abi/damn-vulnerable-defi/DamnValuableNFT.json"
);
sol!(
    #[sol(rpc)]
    ShardsNFTMarketplace,
    "abi/damn-vulnerable-defi/ShardsNFTMarketplace.json"
);
sol!(
    #[sol(rpc)]
    ShardsFeeVault,
    "abi/damn-vulnerable-defi/ShardsFeeVault.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Shards"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.damn_vulnerable_defi.shards.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let some_user = &roles.some_user;
        let provider = roles.provider();

        println!("Deploying the Shards challenge...");

        let nft = DamnValuableNFT::deploy(provider).await?;
        // Mint NFTs: 1-100 to seller, 101-115 to deployer
        for i in 0..115 {
            let to =
                if i < 100 { some_user.address() } else { deployer.address() };
            nft.safeMint(to)
                .from(deployer.address())
                .send()
                .await?
                .watch()
                .await?;
        }

        let token = DamnValuableToken::deploy(provider).await?;

        // Deploy fee vault, then marketplace
        let fee_vault_impl = ShardsFeeVault::deploy(provider).await?;
        let marketplace = ShardsNFTMarketplace::deploy(
            provider,
            *nft.address(),
            *token.address(),
            *fee_vault_impl.address(),
            deployer.address(),    // oracle
            U256::from(1_000_000), // rate
        )
        .await?;

        // Fund seller with tokens
        token
            .transfer(some_user.address(), to_ether(10_000))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        Ok(Target { address: *marketplace.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let marketplace =
            ShardsNFTMarketplace::new(self.address, provider.clone());

        // TODO: determine exact check from upstream test
        let _ = marketplace;
        Ok(false)
    }
}
