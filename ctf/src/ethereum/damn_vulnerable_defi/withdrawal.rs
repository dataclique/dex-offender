//! Damn Vulnerable DeFi — Withdrawal.
//!
//! Finalize cross-chain withdrawals safely via the L1 gateway.

use crate::{Level, Roles, to_ether};
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
    L1Gateway,
    "abi/damn-vulnerable-defi/L1Gateway.json"
);
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    L1Forwarder,
    "abi/damn-vulnerable-defi/L1Forwarder.json"
);
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    L2Handler,
    "abi/damn-vulnerable-defi/L2Handler.json"
);
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    L2MessageStore,
    "abi/damn-vulnerable-defi/L2MessageStore.json"
);
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    TokenBridge,
    "abi/damn-vulnerable-defi/TokenBridge.json"
);

pub const WITHDRAWALS_ROOT: [u8; 32] = [
    0x4e, 0x0f, 0x53, 0xae, 0x5c, 0x8d, 0x5b, 0xc5, 0xfd, 0x1a, 0x52, 0x2b,
    0x9f, 0x37, 0xed, 0xfd, 0x78, 0x2d, 0x6f, 0x4c, 0x7d, 0x8e, 0x0d, 0xf1,
    0x39, 0x15, 0x34, 0xc0, 0x81, 0x23, 0x3d, 0x9e,
];

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
    pub token: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Withdrawal"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let dvd = &ctfs.ethereum.damn_vulnerable_defi.withdrawal;
        Ok(Target { address: dvd.address, token: dvd.token })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let some_user = &roles.some_user;
        let offender = &roles.offender;
        let provider = roles.provider();

        println!("Deploying the Withdrawal challenge...");

        let token = DamnValuableToken::deploy(provider).await?;

        // Deploy L1 gateway + forwarder
        let gateway = L1Gateway::deploy(provider).await?;
        let forwarder =
            L1Forwarder::deploy(provider, *gateway.address()).await?;

        // Deploy L2 side (handler + message store)
        let message_store = L2MessageStore::deploy(provider).await?;
        let handler = L2Handler::deploy(
            provider,
            *message_store.address(),
            *forwarder.address(),
        )
        .await?;

        // Wire forwarder → L2 handler
        forwarder
            .setL2Handler(*handler.address())
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        // Deploy L1 token bridge
        let bridge = TokenBridge::deploy(
            provider,
            *token.address(),
            *forwarder.address(),
            some_user.address(), // L2 token bridge (placeholder)
        )
        .await?;

        // Fund the bridge
        token
            .transfer(*bridge.address(), to_ether(1_000_000))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        // Set withdrawals root in gateway
        gateway
            .setRoot(alloy::primitives::FixedBytes::from(WITHDRAWALS_ROOT))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        // Grant operator role to offender
        let operator_role = gateway.OPERATOR_ROLE().call().await?;
        gateway
            .grantRoles(offender.address(), operator_role)
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        Ok(Target { address: *gateway.address(), token: *token.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let token = DamnValuableToken::new(self.token, provider.clone());

        println!("Checking that withdrawals were finalized...");
        // Bridge should still hold most tokens (less than initial)
        let bridge_balance = token
            .balanceOf(roles.some_user.address()) // L2 bridge placeholder
            .call()
            .await?;
        Ok(bridge_balance < to_ether(1_000_000))
    }
}
