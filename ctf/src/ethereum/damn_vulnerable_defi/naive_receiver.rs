//! Damn Vulnerable DeFi — Naive Receiver.
//!
//! Drain all WETH from the pool and the flash loan receiver in ≤2 txs.

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, primitives::U256, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    WETH,
    "abi/damn-vulnerable-defi/WETH.json"
);
sol!(
    #[sol(rpc)]
    BasicForwarder,
    "abi/damn-vulnerable-defi/BasicForwarder.json"
);
sol!(
    #[sol(rpc)]
    NaiveReceiverPool,
    "abi/damn-vulnerable-defi/NaiveReceiverPool.json"
);
sol!(
    #[sol(rpc)]
    FlashLoanReceiver,
    "abi/damn-vulnerable-defi/FlashLoanReceiver.json"
);

pub const WETH_IN_POOL: u64 = 1000;
pub const WETH_IN_RECEIVER: u64 = 10;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
    pub receiver: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Naive Receiver"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let dvd = &ctfs.ethereum.damn_vulnerable_defi.naive_receiver;
        Ok(Target { address: dvd.address, receiver: dvd.receiver })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        println!("Deploying the Naive Receiver challenge...");

        let weth = WETH::deploy(provider).await?;
        let forwarder = BasicForwarder::deploy(provider).await?;

        let pool_addr = NaiveReceiverPool::deploy_builder(
            provider,
            *forwarder.address(),
            *weth.address(),
            deployer.address(),
        )
        .value(to_ether(WETH_IN_POOL))
        .from(deployer.address())
        .deploy()
        .await?;

        let receiver = FlashLoanReceiver::deploy(provider, pool_addr).await?;

        // Fund the receiver with WETH
        weth.deposit()
            .value(to_ether(WETH_IN_RECEIVER))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        weth.transfer(*receiver.address(), to_ether(WETH_IN_RECEIVER))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        Ok(Target { address: pool_addr, receiver: *receiver.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let pool = NaiveReceiverPool::new(self.address, provider.clone());

        println!("Checking that the pool and receiver are drained...");
        let weth_addr = pool.weth().call().await?;
        let weth = WETH::new(weth_addr, provider.clone());

        let pool_balance = weth.balanceOf(self.address).call().await?;
        let receiver_balance = weth.balanceOf(self.receiver).call().await;

        // Both pool and receiver must be empty
        let receiver_empty = match receiver_balance {
            Ok(b) => b == U256::ZERO,
            Err(_) => true, // receiver self-destructed
        };

        Ok(pool_balance == U256::ZERO && receiver_empty)
    }
}
