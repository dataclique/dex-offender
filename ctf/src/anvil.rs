//! Local-anvil cheat RPC helpers (the harness equivalent of Foundry
//! `vm.deal`/`vm.etch`), used to plant canonical mainnet contracts —
//! e.g. WETH9 at `0xC02aaA…` — that old CTF setups call at fixed
//! addresses.

use crate::Roles;
use alloy::{
    primitives::{Address, Bytes, U256, address},
    providers::Provider,
    rpc::types::BlockId,
};

/// Overwrite the runtime code at `addr` (anvil `setCode`).
pub async fn set_code(
    roles: &Roles,
    addr: Address,
    runtime_code: Bytes,
) -> eyre::Result<()> {
    roles
        .provider()
        .client()
        .request::<_, ()>("anvil_setCode", (addr, runtime_code))
        .await?;
    Ok(())
}

/// Set an account's ETH balance (anvil `setBalance`).
pub async fn set_balance(
    roles: &Roles,
    addr: Address,
    wei: U256,
) -> eyre::Result<()> {
    roles
        .provider()
        .client()
        .request::<_, ()>("anvil_setBalance", (addr, wei))
        .await?;
    Ok(())
}

/// Canonical mainnet WETH9 (what every Paradigm 2021 setup hardcodes).
pub const MAINNET_WETH9: Address =
    address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");

/// Plant a functional WETH9 at the canonical mainnet address.
///
/// Runtime bytecode is compiled from the Uniswap v2-periphery WETH9
/// source (pinned in the damn-vulnerable-defi submodule) and shipped as
/// the forge artifact `ctf/abi/paradigm-2021/weth9.json`.
pub async fn plant_weth9(roles: &Roles) -> eyre::Result<()> {
    // Already planted? Skip so repeated deployments stay idempotent.
    let existing = roles
        .provider()
        .get_code_at(MAINNET_WETH9)
        .block_id(BlockId::latest())
        .await?;
    if !existing.is_empty() {
        return Ok(());
    }

    let artifact: serde_json::Value =
        serde_json::from_str(include_str!("../abi/paradigm-2021/weth9.json"))?;
    let hex =
        artifact["deployedBytecode"]["object"].as_str().ok_or_else(|| {
            eyre::eyre!("weth9 artifact missing deployedBytecode.object")
        })?;
    let code: Bytes = hex.parse()?;

    set_code(roles, MAINNET_WETH9, code).await
}
