//! Ethernaut 24 — Puzzle Wallet.
//!
//! Goal: Become the `admin` of `PuzzleProxy`, the proxy that runs the
//! `PuzzleWallet` implementation.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/PuzzleWallet.sol

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, primitives::U256, sol, sol_types::SolCall};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    PuzzleWallet,
    "../lib/ethernaut/contracts/out/PuzzleWallet.sol/PuzzleWallet.json"
);

sol!(
    #[sol(rpc)]
    PuzzleProxy,
    "../lib/ethernaut/contracts/out/PuzzleWallet.sol/PuzzleProxy.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "PuzzleWallet"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level24.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        // Mirror PuzzleWalletFactory.createInstance exactly:
        // 1. deploy the PuzzleWallet logic
        let logic = PuzzleWallet::deploy(provider).await?;

        // 2. deploy proxy with init(100 ether) as upgrade data
        let init_data =
            PuzzleWallet::initCall { _maxBalance: to_ether(100) }.abi_encode();
        let proxy_addr = PuzzleProxy::deploy_builder(
            provider,
            deployer.address(),
            *logic.address(),
            init_data.into(),
        )
        .deploy()
        .await?;

        // 3. whitelist the factory (deployer here) and deposit 0.001 ETH
        let wallet = PuzzleWallet::new(proxy_addr, provider.clone());
        wallet
            .addToWhitelist(deployer.address())
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        wallet
            .deposit()
            .value(to_ether(1) / U256::from(1000))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        println!("Deployed the PuzzleWallet proxy at {proxy_addr}");
        Ok(Target { address: proxy_addr })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();

        let proxy = PuzzleProxy::new(self.address, provider.clone());
        let admin = proxy.admin().call().await?;
        Ok(admin == roles.offender.address())
    }
}
