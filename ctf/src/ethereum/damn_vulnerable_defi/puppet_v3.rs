//! Damn Vulnerable DeFi — PuppetV3.
//!
//! Drain the PuppetV3Pool by manipulating the UniswapV3 price.
//!
//! BLOCKED: requires mainnet fork (real Uniswap V3 factory + WETH +
//! PositionManager at fork block 15450164).

use crate::{Level, Roles};
use alloy::{primitives::Address, sol};
use async_trait::async_trait;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    DamnValuableToken,
    "abi/damn-vulnerable-defi/DamnValuableToken.json"
);
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    PuppetV3Pool,
    "abi/damn-vulnerable-defi/PuppetV3Pool.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
    pub token: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "PuppetV3"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let dvd = ctfs
            .ethereum
            .damn_vulnerable_defi
            .puppet_v3
            .as_ref()
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?;
        Ok(Target { address: dvd.address, token: dvd.token })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let _ = roles;
        unimplemented!(
            "PuppetV3 requires mainnet fork at block 15450164 (real Uniswap V3 factory)"
        )
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let token = DamnValuableToken::new(self.token, provider.clone());

        println!("Checking that the pool is drained...");
        Ok(token.balanceOf(self.address).call().await?
            == alloy::primitives::U256::ZERO)
    }
}
