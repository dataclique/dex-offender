//! Ethernaut 37 — Impersonator Two.
//!
//! Goal: Empty the Ether held by the locked `ImpersonatorTwo` contract, which
//! starts with 0.001 ETH.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/ImpersonatorTwo.sol

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, primitives::U256, providers::Provider, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    ImpersonatorTwo,
    "../lib/ethernaut/contracts/out/ImpersonatorTwo.sol/ImpersonatorTwo.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Impersonator Two"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level37.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        // Mirror ImpersonatorTwoFactory: fund 0.001 ETH at construction,
        // hand ownership to the level's fixed owner, lock the funds, and
        // set the level's fixed admin with the factory's known signatures.
        let instance = ImpersonatorTwo::deploy_builder(provider)
            .value(to_ether(1) / U256::from(1000))
            .from(deployer.address())
            .deploy()
            .await?;
        let contract = ImpersonatorTwo::new(instance, provider.clone());

        let owner = alloy::primitives::address!(
            "03E2cf81BBE61D1fD1421aFF98e8605a5A9e953a"
        );
        let admin = alloy::primitives::address!(
            "ADa4aFfe581d1A31d7F75E1c5a3A98b2D4C40f68"
        );
        contract
            .transferOwnership(owner)
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        let mut switch_sig = Vec::with_capacity(65);
        switch_sig.extend_from_slice(&alloy::primitives::hex::decode(
            "e5648161e95dbf2bfc687b72b745269fa906031e2108118050aba59524a23c40",
        )?);
        switch_sig.extend_from_slice(&alloy::primitives::hex::decode(
            "70026fc30e4e02a15468de57155b080f405bd5b88af05412a9c3217e028537e3",
        )?);
        switch_sig.push(27);
        contract
            .switchLock(switch_sig.into())
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        let mut admin_sig = Vec::with_capacity(65);
        admin_sig.extend_from_slice(&alloy::primitives::hex::decode(
            "e5648161e95dbf2bfc687b72b745269fa906031e2108118050aba59524a23c40",
        )?);
        admin_sig.extend_from_slice(&alloy::primitives::hex::decode(
            "4c3ac03b268ae1d2aca1201e8a936adf578a8b95a49986d54de87cd0ccb68a79",
        )?);
        admin_sig.push(27);
        contract
            .setAdmin(admin_sig.into(), admin)
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        Ok(Target { address: instance })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();

        let balance = provider.get_balance(self.address).await?;
        Ok(balance == U256::ZERO)
    }
}
