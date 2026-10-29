//! Paradigm CTF 2023 — Enterprise Blockchain.
//!
//! Exploit the cross-chain bridge.

use crate::{Level, Roles};
use alloy::{primitives::Address, primitives::U256, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    Bridge,
    "abi/paradigm-2023/Bridge.json"
);

sol!(
    #[sol(rpc)]
    FlagToken,
    "abi/paradigm-2023/FlagToken.json"
);

sol!(
    #[sol(rpc)]
    Challenge,
    "abi/paradigm-2023/enterprise-blockchain.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Enterprise Blockchain"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs
            .ethereum
            .paradigm_2023
            .enterprise_blockchain
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?
            .address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let relayer = &roles.some_user;
        let offender = &roles.offender;
        let provider = roles.provider();

        // Mirror the upstream Deploy script. It forks L1+L2; locally both
        // bridges live on the same anvil with distinct registered chain ids
        // (78705 self-registration, 78704 → L1 bridge), preserving the
        // replay mechanics.
        let l1_bridge = Bridge::deploy(provider, relayer.address()).await?;
        let flag_token = FlagToken::deploy(
            provider,
            *l1_bridge.address(),
            offender.address(),
        )
        .await?;
        let contract = Challenge::deploy(
            provider,
            *l1_bridge.address(),
            *flag_token.address(),
        )
        .await?;

        l1_bridge
            .registerRemoteBridge(U256::from(78705), *l1_bridge.address())
            .from(relayer.address())
            .send()
            .await?
            .watch()
            .await?;

        let l2_bridge = Bridge::deploy(provider, relayer.address()).await?;
        l2_bridge
            .registerRemoteBridge(U256::from(78704), *l1_bridge.address())
            .from(relayer.address())
            .send()
            .await?
            .watch()
            .await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = Challenge::new(self.address, provider.clone());

        let solved = contract.isSolved().call().await?;
        Ok(solved)
    }
}
