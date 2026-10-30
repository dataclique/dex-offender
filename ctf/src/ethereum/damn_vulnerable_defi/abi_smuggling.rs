//! Damn Vulnerable DeFi — ABI Smuggling.
//!
//! Exploit the selector clash to sweep the vault.

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
    SelfAuthorizedVault,
    "../lib/damn-vulnerable-defi/out/SelfAuthorizedVault.sol/SelfAuthorizedVault.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
    pub token: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "ABI Smuggling"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let dvd = &ctfs.ethereum.damn_vulnerable_defi.abi_smuggling;
        Ok(Target { address: dvd.address, token: dvd.token })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let offender = &roles.offender;
        let provider = roles.provider();

        println!("Deploying the ABI Smuggling challenge...");

        let token = DamnValuableToken::deploy(provider).await?;
        let vault = SelfAuthorizedVault::deploy(provider).await?;

        // Set permissions for deployer (sweep) and offender (execute)
        let deployer_perm = vault
            .getActionId(
                alloy::primitives::FixedBytes::from([0x85, 0xfb, 0x70, 0x9d]),
                deployer.address(),
                *vault.address(),
            )
            .call()
            .await?;
        let player_perm = vault
            .getActionId(
                alloy::primitives::FixedBytes::from([0xd9, 0xca, 0xed, 0x12]),
                offender.address(),
                *vault.address(),
            )
            .call()
            .await?;
        vault
            .setPermissions(vec![deployer_perm, player_perm])
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        token
            .transfer(*vault.address(), to_ether(1_000_000))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        Ok(Target { address: *vault.address(), token: *token.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let token = DamnValuableToken::new(self.token, provider.clone());

        Ok(token.balanceOf(self.address).call().await? == U256::ZERO)
    }
}
