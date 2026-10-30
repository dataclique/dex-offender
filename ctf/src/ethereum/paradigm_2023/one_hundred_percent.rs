//! Paradigm CTF 2023 — One Hundred Percent.
//!
//! Get 100% of the split.

use crate::{Level, Roles, to_ether};
use alloy::{
    primitives::U256,
    primitives::{Address, address},
    sol,
};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    Split,
    "../lib/paradigm-ctf-2023/one-hundred-percent/challenge/project/out/Split.sol/Split.json"
);

sol!(
    #[sol(rpc)]
    SplitWallet,
    "../lib/paradigm-ctf-2023/one-hundred-percent/challenge/project/out/SplitWallet.sol/SplitWallet.json"
);

sol!(
    #[sol(rpc)]
    Challenge,
    "../lib/paradigm-ctf-2023/one-hundred-percent/challenge/project/out/Challenge.sol/Challenge.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "One Hundred Percent"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs
            .ethereum
            .paradigm_2023
            .one_hundred_percent
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?
            .address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        // Mirror the upstream Deploy script: Split, a 50/50 dEaD/bEEF
        // split, 100 ether deposited into its wallet, then the Challenge.
        let split = Split::deploy(provider).await?;
        let id = split
            .createSplit(
                vec![
                    address!("000000000000000000000000000000000000dEaD"),
                    address!("000000000000000000000000000000000000bEEF"),
                ],
                vec![500_000, 500_000],
                0,
            )
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        let _ = id;

        let data = split.splitsById(U256::ZERO).call().await?;
        SplitWallet::new(data.wallet, provider.clone())
            .deposit()
            .value(to_ether(100))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        let contract = Challenge::deploy(provider, *split.address()).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = Challenge::new(self.address, provider.clone());

        let solved = contract.isSolved().call().await?;
        Ok(solved)
    }
}
