//! Damn Vulnerable DeFi — Compromised.
//!
//! Compromise the oracle sources to manipulate NFT prices and drain the
//! exchange.

use crate::{Level, Roles, to_ether};
use alloy::{
    primitives::{Address, U256, address},
    providers::Provider,
    sol,
};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    TrustfulOracleInitializer,
    "../lib/damn-vulnerable-defi/out/TrustfulOracleInitializer.sol/TrustfulOracleInitializer.json"
);
sol!(
    #[sol(rpc)]
    TrustfulOracle,
    "../lib/damn-vulnerable-defi/out/TrustfulOracle.sol/TrustfulOracle.json"
);
sol!(
    #[sol(rpc)]
    Exchange,
    "../lib/damn-vulnerable-defi/out/Exchange.sol/Exchange.json"
);
sol!(
    #[sol(rpc)]
    DamnValuableNFT,
    "../lib/damn-vulnerable-defi/out/DamnValuableNFT.sol/DamnValuableNFT.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Compromised"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.damn_vulnerable_defi.compromised.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        println!("Deploying the Compromised challenge...");

        let sources = vec![
            address!("188Ea627E3531Db590e6f1D71ED83628d1933088"),
            address!("A417D473c40a4d42BAd35f147c21eEa7973539D8"),
            address!("ab3600bF153A316dE44827e2473056d56B774a40"),
        ];
        let symbols =
            vec!["DVNFT".to_string(), "DVNFT".to_string(), "DVNFT".to_string()];
        let prices = vec![to_ether(999), to_ether(999), to_ether(999)];

        let initializer = TrustfulOracleInitializer::deploy(
            provider, sources, symbols, prices,
        )
        .await?;
        let oracle_addr = initializer.oracle().call().await?;

        let exchange_addr = Exchange::deploy_builder(provider, oracle_addr)
            .value(to_ether(999))
            .from(deployer.address())
            .deploy()
            .await?;

        Ok(Target { address: exchange_addr })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let _exchange = Exchange::new(self.address, provider.clone());

        let balance = provider.get_balance(self.address).await?;

        // Exchange must have 0 ETH
        Ok(balance == U256::ZERO)
    }
}
