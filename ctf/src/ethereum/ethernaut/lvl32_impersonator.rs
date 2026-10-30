//! Ethernaut 32 — Impersonator.
//!
//! Goal: Clear the controller of the first `ECLocker` created by
//! `Impersonator`, leaving `lockers(0).controller()` equal to the zero address.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Impersonator.sol

use crate::{Level, Roles};
use alloy::{primitives::Address, primitives::U256, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    Impersonator,
    "../lib/ethernaut/contracts/out/Impersonator.sol/Impersonator.json"
);

sol!(
    #[sol(rpc)]
    ECLocker,
    "../lib/ethernaut/contracts/out/Impersonator.sol/ECLocker.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Impersonator"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level32.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        // Mirror ImpersonatorFactory: Impersonator(1336) plus the first
        // ECLock deployed with the factory's known signature (r, s, v).
        let contract =
            Impersonator::deploy(provider, alloy::primitives::U256::from(1336))
                .await?;
        let r: alloy::primitives::U256 =
            "11397568185806560130291530949248708355673262872727946990834312389557386886033"
                .parse()?;
        let s: alloy::primitives::U256 =
            "54405834204020870944342294544757609285398723182661749830189277079337680158706"
                .parse()?;
        let mut sig = Vec::with_capacity(96);
        sig.extend_from_slice(&r.to_be_bytes::<32>());
        sig.extend_from_slice(&s.to_be_bytes::<32>());
        sig.extend_from_slice(
            &alloy::primitives::U256::from(27).to_be_bytes::<32>(),
        );
        contract
            .deployNewLock(sig.into())
            .from(roles.deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = Impersonator::new(self.address, provider.clone());

        // Mirror validateInstance: lockers(0).controller() == 0.
        let locker_addr = contract.lockers(U256::from(0)).call().await?;
        let locker = ECLocker::new(locker_addr, provider.clone());
        let controller = locker.controller().call().await?;
        Ok(controller == Address::ZERO)
    }
}
