//! Ethernaut 17 — Recovery.
//!
//! Goal: Recover all Ether from the lost `SimpleToken` created by `Recovery`.
//! The token's Ether balance, not the factory's, must become zero.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/Recovery.sol

use crate::{Level, Roles};
use alloy::{
    primitives::{Address, U256, keccak256},
    providers::Provider,
    sol,
};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    Recovery,
    "../lib/ethernaut/contracts/out/Recovery.sol/Recovery.json"
);

/// Ether the factory's createInstance seeds the lost token with (its
/// minimum msg.value).
const LOST_TOKEN_FUNDING: u64 = 1_000_000_000_000; // 0.001 ether

/// The lost `SimpleToken` address: the Recovery instance's second
/// contract creation (nonce 1) — the same packed CREATE derivation the
/// factory records (0xd6 0x94 ‖ address ‖ nonce, keccak, last 20 bytes).
fn lost_token_address(recovery: Address) -> Address {
    let mut payload = Vec::with_capacity(23);
    payload.push(0xd6);
    payload.push(0x94);
    payload.extend_from_slice(recovery.as_slice());
    payload.push(0x01);
    Address::from_word(keccak256(payload))
}

/// Derived lost-token address (shared with the level regression).
pub fn lost_token_address_for_test(recovery: Address) -> Address {
    lost_token_address(recovery)
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Recovery"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level17.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        println!("Deploying the Recovery contract...");
        let contract = Recovery::deploy(provider).await?;

        // Mirror RecoveryFactory.createInstance: generate the token
        // (the instance's creation nonce 1), then fund that CREATE
        // address with the factory's minimum value.
        contract
            .generateToken(String::from("InitialToken"), U256::from(100_000))
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        let lost = lost_token_address(*contract.address());
        deployer.transfer(lost, U256::from(LOST_TOKEN_FUNDING)).await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let lost = lost_token_address(self.address);

        println!(
            "Checking that you found the token and took all ether from it..."
        );
        let balance = provider.get_balance(lost).await?;
        Ok(balance.is_zero())
    }
}
