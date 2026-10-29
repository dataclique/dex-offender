//! Damn Vulnerable DeFi — Withdrawal.
//!
//! Finalize cross-chain withdrawals safely via the L1 gateway.

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, primitives::U256, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    DamnValuableToken,
    "abi/damn-vulnerable-defi/DamnValuableToken.json"
);
sol!(
    #[sol(rpc)]
    L1Gateway,
    "abi/damn-vulnerable-defi/L1Gateway.json"
);
sol!(
    #[sol(rpc)]
    L1Forwarder,
    "abi/damn-vulnerable-defi/L1Forwarder.json"
);
sol!(
    #[sol(rpc)]
    L2Handler,
    "abi/damn-vulnerable-defi/L2Handler.json"
);
sol!(
    #[sol(rpc)]
    L2MessageStore,
    "abi/damn-vulnerable-defi/L2MessageStore.json"
);
sol!(
    #[sol(rpc)]
    TokenBridge,
    "abi/damn-vulnerable-defi/TokenBridge.json"
);

pub const WITHDRAWALS_ROOT: [u8; 32] = [
    0x4e, 0x0f, 0x53, 0xae, 0x5c, 0x8d, 0x5b, 0xc5, 0xfd, 0x1a, 0x52, 0x2b,
    0x9f, 0x37, 0xed, 0xfd, 0x78, 0x2d, 0x6f, 0x4c, 0x7d, 0x8e, 0x0d, 0xf1,
    0x39, 0x15, 0x34, 0xc0, 0x81, 0x23, 0x3d, 0x9e,
];

impl Target {
    /// The four withdrawal hashes the challenge must finalize, straight
    /// from the upstream test's `_isSolved`.
    const WITHDRAWALS: [[u8; 32]; 4] = [
        [
            0xea, 0xeb, 0xef, 0x7f, 0x15, 0xfd, 0xaa, 0x66, 0xec, 0xd4, 0x53,
            0x3e, 0xef, 0xea, 0x23, 0xa1, 0x83, 0xce, 0xd2, 0x99, 0x67, 0xea,
            0x67, 0xbc, 0x42, 0x19, 0xb0, 0xf1, 0xf8, 0xb0, 0xd3, 0xba,
        ],
        [
            0x0b, 0x13, 0x01, 0x75, 0xae, 0xb6, 0x13, 0x0c, 0x81, 0x83, 0x9d,
            0x7a, 0xd4, 0xf5, 0x80, 0xcd, 0x18, 0x93, 0x1c, 0xaf, 0x17, 0x77,
            0x93, 0xcd, 0x3b, 0xab, 0x95, 0xb8, 0xcb, 0xb8, 0xde, 0x60,
        ],
        [
            0xba, 0xee, 0x8d, 0xea, 0x6b, 0x24, 0xd3, 0x27, 0xbc, 0x9f, 0xcd,
            0x7c, 0xe8, 0x67, 0x99, 0x04, 0x27, 0xb9, 0xd6, 0xf4, 0x8a, 0x92,
            0xf4, 0xb3, 0x31, 0x51, 0x4e, 0xa6, 0x88, 0x90, 0x90, 0x15,
        ],
        [
            0x9a, 0x8d, 0xbc, 0xcb, 0x61, 0x71, 0xdc, 0x54, 0xbf, 0xcf, 0xf6,
            0x47, 0x1f, 0x41, 0x94, 0x71, 0x66, 0x88, 0x61, 0x93, 0x05, 0xb6,
            0xed, 0xed, 0xc5, 0x41, 0x08, 0xec, 0x35, 0xb3, 0x9b, 0x09,
        ],
    ];
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
    pub token: Address,
    pub bridge: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Withdrawal"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let dvd = &ctfs.ethereum.damn_vulnerable_defi.withdrawal;
        Ok(Target {
            address: dvd.address,
            token: dvd.token,
            bridge: dvd.bridge,
        })
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

        Ok(Target {
            address: *gateway.address(),
            token: *token.address(),
            bridge: *bridge.address(),
        })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let token = DamnValuableToken::new(self.token, provider.clone());
        let gateway = L1Gateway::new(self.address, provider.clone());

        println!("Checking that withdrawals were finalized...");
        let bridge_balance = token.balanceOf(self.bridge).call().await?;
        if !(bridge_balance > to_ether(990_000)
            && bridge_balance < to_ether(1_000_000))
        {
            return Ok(false);
        }
        if token.balanceOf(roles.offender.address()).call().await?
            != U256::from(0)
        {
            return Ok(false);
        }
        if gateway.counter().call().await? < U256::from(4) {
            return Ok(false);
        }
        for hash in Self::WITHDRAWALS {
            if !gateway
                .finalizedWithdrawals(alloy::primitives::B256::from(hash))
                .call()
                .await?
            {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
