//! Paradigm CTF 2023 — Free Real Estate.
//!
//! Drain the airdrop distributor.

use crate::{Level, Roles};
use alloy::{
    primitives::{Address, U256, b256},
    sol,
};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    InuToken,
    "abi/paradigm-2023/InuToken.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    MerkleDistributor,
    "abi/paradigm-2023/MerkleDistributor.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Challenge,
    "abi/paradigm-2023/free-real-estate.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Free Real Estate"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs
            .ethereum
            .paradigm_2023
            .free_real_estate
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?
            .address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        // Mirror the upstream Deploy script with the pinned airdrop data
        // (challenge/airdrop-merkle-proofs.json.gz): root + total supply.
        let merkle_root = b256!(
            "43c82b918f439b8608d0624cc9a557518faecb07d399460a737c4f9b62b32591"
        );
        let raw = alloy::primitives::hex::decode("b16c7747f6a745eab00000")?;
        let mut total_bytes = [0u8; 32];
        total_bytes[32 - raw.len()..].copy_from_slice(&raw);
        let token_total = U256::from_be_bytes(total_bytes);

        let token = InuToken::deploy(provider, token_total).await?;
        let distributor =
            MerkleDistributor::deploy(provider, *token.address(), merkle_root)
                .await?;
        token
            .transfer(*distributor.address(), token_total)
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        let contract =
            Challenge::deploy(provider, *distributor.address()).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = Challenge::new(self.address, provider.clone());

        let solved = contract.getScore().call().await? > 0;
        Ok(solved)
    }
}
