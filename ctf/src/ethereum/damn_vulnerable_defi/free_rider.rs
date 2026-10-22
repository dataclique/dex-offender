//! Damn Vulnerable DeFi — FreeRider.
//!
//! Buy all NFTs for the price of one using a flash loan.

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, primitives::U256, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    DamnValuableToken,
    "abi/damn-vulnerable-defi/DamnValuableToken.json"
);
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    DamnValuableNFT,
    "abi/damn-vulnerable-defi/DamnValuableNFT.json"
);
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    WETH,
    "abi/damn-vulnerable-defi/WETH.json"
);
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UniswapV2Factory,
    "abi/damn-vulnerable-defi/UniswapV2Factory.json"
);
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UniswapV2Router02,
    "abi/damn-vulnerable-defi/UniswapV2Router02.json"
);
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    FreeRiderNFTMarketplace,
    "abi/damn-vulnerable-defi/FreeRiderNFTMarketplace.json"
);
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    FreeRiderRecoveryManager,
    "abi/damn-vulnerable-defi/FreeRiderRecoveryManager.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
    pub token: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "FreeRider"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let dvd = &ctfs.ethereum.damn_vulnerable_defi.free_rider;
        Ok(Target { address: dvd.address, token: dvd.token })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let offender = &roles.offender;
        let provider = roles.provider();

        println!("Deploying the FreeRider challenge...");

        let token = DamnValuableToken::deploy(provider).await?;
        let weth = WETH::deploy(provider).await?;
        let factory = UniswapV2Factory::deploy(provider, Address::ZERO).await?;
        let router = UniswapV2Router02::deploy(
            provider,
            *factory.address(),
            *weth.address(),
        )
        .await?;

        // Add liquidity: 15000 DVT + 9000 WETH (use some_user — deployer
        // is depleted by prior levels)
        let liquidity_provider = &roles.some_user;
        token
            .transfer(liquidity_provider.address(), to_ether(15_000))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        token
            .approve(*router.address(), to_ether(15_000))
            .from(liquidity_provider.address())
            .send()
            .await?
            .watch()
            .await?;
        router
            .addLiquidityETH(
                *token.address(),
                to_ether(15_000),
                U256::ZERO,
                U256::ZERO,
                liquidity_provider.address(),
                U256::MAX,
            )
            .value(to_ether(9000))
            .from(liquidity_provider.address())
            .send()
            .await?
            .watch()
            .await?;

        // Deploy marketplace with ETH
        let marketplace_addr =
            FreeRiderNFTMarketplace::deploy_builder(provider, U256::from(6))
                .value(to_ether(90))
                .from(deployer.address())
                .deploy()
                .await?;
        let marketplace =
            FreeRiderNFTMarketplace::new(marketplace_addr, provider.clone());

        // Get NFT address and deploy recovery manager
        let nft_addr = marketplace.token().call().await?;
        let _nft = DamnValuableNFT::new(nft_addr, provider.clone());
        let recovery = FreeRiderRecoveryManager::deploy_builder(
            provider,
            offender.address(),
            nft_addr,
            deployer.address(),
            to_ether(45),
        )
        .value(to_ether(45))
        .from(deployer.address())
        .deploy()
        .await?;
        let _ = recovery;

        Ok(Target { address: marketplace_addr, token: *token.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let marketplace =
            FreeRiderNFTMarketplace::new(self.address, provider.clone());

        println!("Checking that all NFTs are sold...");
        let offers_count = marketplace.offersCount().call().await?;

        Ok(offers_count == U256::ZERO)
    }
}
