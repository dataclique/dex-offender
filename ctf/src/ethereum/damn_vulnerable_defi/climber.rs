//! Damn Vulnerable DeFi — Climber.
//!
//! Drain the ClimberVault by exploiting the timelock's scheduling logic.

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, primitives::U256, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    DamnValuableToken,
    "../lib/damn-vulnerable-defi/out/DamnValuableToken.sol/DamnValuableToken.json"
);
sol!(
    #[sol(rpc)]
    ClimberVault,
    "../lib/damn-vulnerable-defi/out/ClimberVault.sol/ClimberVault.json"
);
sol!(
    #[sol(rpc)]
    ClimberTimelock,
    "../lib/damn-vulnerable-defi/out/ClimberTimelock.sol/ClimberTimelock.json"
);
sol!(
    #[sol(rpc)]
    ERC1967Proxy,
    "../lib/damn-vulnerable-defi/out/ERC1967Proxy.sol/ERC1967Proxy.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
    pub token: Address,
    pub timelock: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Climber"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let dvd = &ctfs.ethereum.damn_vulnerable_defi.climber;
        Ok(Target {
            address: dvd.address,
            token: dvd.token,
            timelock: dvd.timelock,
        })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let some_user = &roles.some_user;
        let provider = roles.provider();

        println!("Deploying the Climber challenge...");

        // Deploy vault implementation
        let vault_impl = ClimberVault::deploy(provider).await?;

        // Encode initialize(deployer, proposer, sweeper) calldata
        let init_selector = alloy::primitives::keccak256(
            "initialize(address,address,address)".as_bytes(),
        );
        let init_data = alloy::primitives::Bytes::from(
            [
                &init_selector[..4] as &[u8],
                &[0u8; 12] as &[u8],
                deployer.address().as_slice(),
                &[0u8; 12] as &[u8],
                some_user.address().as_slice(),
                &[0u8; 12] as &[u8],
                roles.offender.address().as_slice(),
            ]
            .concat(),
        );

        // Deploy proxy with vault implementation + init data
        let proxy_addr = ERC1967Proxy::deploy_builder(
            provider,
            *vault_impl.address(),
            init_data,
        )
        .from(deployer.address())
        .deploy()
        .await?;
        let vault = ClimberVault::new(proxy_addr, provider.clone());

        // Get timelock (it's the vault's owner)
        let timelock_addr = vault.owner().call().await?;

        // Deploy token and fund vault
        let token = DamnValuableToken::deploy(provider).await?;
        token
            .transfer(proxy_addr, to_ether(10_000_000))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        Ok(Target {
            address: proxy_addr,
            token: *token.address(),
            timelock: timelock_addr,
        })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let token = DamnValuableToken::new(self.token, provider.clone());

        Ok(token.balanceOf(self.address).call().await? == U256::ZERO)
    }
}
