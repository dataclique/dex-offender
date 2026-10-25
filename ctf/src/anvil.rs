//! Local-anvil cheat RPC helpers (the harness equivalent of Foundry
//! `vm.deal`/`vm.etch`), used to plant canonical mainnet contracts —
//! e.g. WETH9 at `0xC02aaA…` — that old CTF setups call at fixed
//! addresses.

use crate::Roles;
use alloy::{
    network::{ReceiptResponse, TransactionBuilder},
    primitives::{Address, B256, Bytes, U256, address},
    providers::Provider,
    rpc::types::BlockId,
    sol,
    sol_types::SolValue,
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

/// Write a raw storage slot (anvil `setStorageAt`).
pub async fn set_storage(
    roles: &Roles,
    addr: Address,
    slot: U256,
    value: B256,
) -> eyre::Result<()> {
    roles
        .provider()
        .client()
        .request::<_, bool>("anvil_setStorageAt", (addr, slot, value))
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

/// Canonical mainnet Chainlink ETH/USD price feed.
pub const MAINNET_ETH_USD_FEED: Address =
    address!("5f4eC3Df9cbd43714FE2740f5E3616155c5b8419");

/// Plant a fixed-price (2000 USD) Chainlink mock at the canonical
/// mainnet feed address (used by Paradigm 2023 dai-plus-plus).
/// Idempotent.
pub async fn plant_eth_usd_feed(roles: &Roles) -> eyre::Result<()> {
    let existing = roles
        .provider()
        .get_code_at(MAINNET_ETH_USD_FEED)
        .block_id(BlockId::latest())
        .await?;
    if !existing.is_empty() {
        return Ok(());
    }

    let artifact: serde_json::Value = serde_json::from_str(include_str!(
        "../abi/paradigm-2023/MockEthUsdFeed.json"
    ))?;
    let hex =
        artifact["deployedBytecode"]["object"].as_str().ok_or_else(|| {
            eyre::eyre!("mock feed artifact missing deployedBytecode.object")
        })?;
    let code: Bytes = hex.parse()?;

    set_code(roles, MAINNET_ETH_USD_FEED, code).await
}

/// Canonical mainnet UniswapV2 factory.
pub const MAINNET_UNISWAP_V2_FACTORY: Address =
    address!("5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f");

/// Canonical mainnet COMP token.
pub const MAINNET_COMP: Address =
    address!("c00e94Cb662C3520282E6f5717214004A7f26888");

/// Canonical mainnet DAI token.
pub const MAINNET_DAI: Address =
    address!("6B175474E89094C44Da98b954EedeAC495271d0F");

/// Canonical mainnet TrueUSD token.
pub const MAINNET_TUSD: Address =
    address!("0000000000085d4780B73119b644AE5ecd22b376");

/// Canonical mainnet USDC token.
pub const MAINNET_USDC: Address =
    address!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");

/// Canonical mainnet USDT token.
pub const MAINNET_USDT: Address =
    address!("dAC17F958D2ee523a2206206994597C13D831ec7");

/// Canonical mainnet UniswapV2 Router02.
pub const MAINNET_UNISWAP_V2_ROUTER: Address =
    address!("7a250d5630B4cF539739dF2C5dAcb4c659F2488D");

/// Canonical mainnet UniswapV2 Router01 (deprecated on mainnet; the
/// swap challenge hardcodes it).
pub const MAINNET_UNISWAP_V2_ROUTER01: Address =
    address!("f164fC0Ec4E93095b804a4795bBe1e041497b92a");

/// Plant the MockERC20 runtime (built from `.tmp/mock-erc20`, embedded
/// at `abi/paradigm-2021/mock-erc20.json`) at `addr`. Balances are
/// seeded separately via [`seed_erc20_balance`].
pub async fn plant_mock_erc20(
    roles: &Roles,
    addr: Address,
) -> eyre::Result<()> {
    let existing = roles.provider().get_code_at(addr).await?;
    if !existing.is_empty() {
        return Ok(());
    }
    let artifact: serde_json::Value = serde_json::from_str(include_str!(
        "../abi/paradigm-2021/mock-erc20.json"
    ))?;
    // deployedBytecode: planting the creation bytecode would execute its
    // constructor prefix (CODECOPY + RETURN) on every call and return the
    // runtime code instead of the ABI-encoded balance.
    let hex =
        artifact["deployedBytecode"]["object"].as_str().ok_or_else(|| {
            eyre::eyre!("mock erc20 artifact missing deployedBytecode.object")
        })?;
    let code: Bytes = hex.parse()?;
    set_code(roles, addr, code).await
}

/// Seed `holder`'s MockERC20 `balanceOf` (mapping slot 0) and keep
/// `totalSupply` (slot 2) in sync by adding `amount`.
pub async fn seed_erc20_balance(
    roles: &Roles,
    token: Address,
    holder: Address,
    amount: U256,
) -> eyre::Result<()> {
    // Additive, not overwriting: a pair seeded by an earlier level has
    // synced reserves at the old balance; lowering it below the synced
    // reserve makes the next `pair.mint` revert (ds-math-sub-underflow).
    let mock = MockErc20::new(token, roles.provider());
    let current = mock.balanceOf(holder).call().await?;
    let next = current.saturating_add(amount);

    // balanceOf[holder] lives at keccak256(abi.encode(holder, 0))
    let mut key = [0u8; 64];
    key[12..32].copy_from_slice(holder.as_slice());
    let slot = alloy::primitives::keccak256(key);
    set_storage(
        roles,
        token,
        U256::from_be_bytes(slot.0),
        B256::from(next.to_be_bytes()),
    )
    .await
}

/// Plant the UniswapV2 Router02 bytecode at `at_addr` (the canonical
/// Router02 address, or the deprecated Router01 address which the swap
/// challenge hardcodes — `swapExactETHForTokens` is selector-identical
/// across both), deployed against the planted factory and WETH9. The
/// router keeps `factory`/`WETH` as immutables, so planting the patched
/// runtime via `anvil_setCode` needs no storage copying.
pub async fn plant_uniswap_v2_router_at(
    roles: &Roles,
    at_addr: Address,
) -> eyre::Result<Address> {
    let existing = roles.provider().get_code_at(at_addr).await?;
    if !existing.is_empty() {
        return Ok(at_addr);
    }

    let artifact: serde_json::Value = serde_json::from_str(include_str!(
        "../abi/damn-vulnerable-defi/UniswapV2Router02.json"
    ))?;
    let hex = artifact["bytecode"]
        .as_str()
        .ok_or_else(|| eyre::eyre!("router artifact missing bytecode"))?;
    let code: Bytes = hex.parse()?;

    // ctor args: (address _factory, address _WETH), abi-encoded
    let calldata = [
        code.as_ref(),
        &MAINNET_UNISWAP_V2_FACTORY.abi_encode(),
        &MAINNET_WETH9.abi_encode(),
    ]
    .concat();

    let receipt = roles
        .provider()
        .send_transaction(
            alloy::rpc::types::TransactionRequest::default()
                .create()
                .with_from(roles.deployer.address())
                .with_input(calldata),
        )
        .await?
        .get_receipt()
        .await?;
    let deployed = receipt
        .contract_address()
        .ok_or_else(|| eyre::eyre!("router deploy produced no address"))?;

    let runtime = roles.provider().get_code_at(deployed).await?;
    set_code(roles, at_addr, runtime).await?;
    Ok(at_addr)
}

/// Plant the UniswapV2 Router02 at its canonical mainnet address.
pub async fn plant_uniswap_v2_router(roles: &Roles) -> eyre::Result<Address> {
    plant_uniswap_v2_router_at(roles, MAINNET_UNISWAP_V2_ROUTER).await
}

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    V2Factory,
    "abi/damn-vulnerable-defi/UniswapV2Factory.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    MockErc20,
    "abi/paradigm-2021/mock-erc20.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    WETH9Token,
    "abi/paradigm-2021/weth9.json"
);

/// Create the WETH/`token` pair via the planted factory, give it
/// `tokens` of mock-token balance (storage-seeded) and `weth_amount`
/// of real WETH, then mint LP so the pair has usable reserves.
pub async fn seed_weth_pair(
    roles: &Roles,
    token: Address,
    tokens: u128,
    weth_amount: u128,
) -> eyre::Result<()> {
    let deployer = roles.deployer.address();
    let provider = roles.provider();
    let factory = V2Factory::new(MAINNET_UNISWAP_V2_FACTORY, provider.clone());
    let weth = WETH9Token::new(MAINNET_WETH9, provider.clone());

    // Idempotent: a prior level (e.g. farmer) may have created this
    // pair already; createPair reverts with PAIR_EXISTS and the failed
    // gas estimate would poison the wallet nonce cache for every later
    // send.
    let mut pair = factory.getPair(MAINNET_WETH9, token).call().await?;
    if Address::new(pair.0.0).is_zero() {
        factory
            .createPair(MAINNET_WETH9, token)
            .from(deployer)
            .send()
            .await?
            .get_receipt()
            .await?;
        pair = factory.getPair(MAINNET_WETH9, token).call().await?;
    }
    let pair_address = Address::new(pair.0.0);
    if pair_address.is_zero() {
        return Err(eyre::eyre!("createPair produced no pair"));
    }

    seed_erc20_balance(
        roles,
        token,
        pair_address,
        U256::from(tokens) * U256::from(1_000_000_000_000_000_000u128),
    )
    .await?;

    weth.transfer(
        pair_address,
        U256::from(weth_amount) * U256::from(1_000_000_000_000_000_000u128),
    )
    .from(deployer)
    .send()
    .await?
    .get_receipt()
    .await?;

    // pair.mint(to) (selector 0x6a627842): sync reserves from balances
    // and mint LP to the deployer.
    let mut mint_calldata = alloy::primitives::hex!("6a627842").to_vec();
    mint_calldata.extend_from_slice(&deployer.abi_encode());
    provider
        .send_transaction(
            alloy::rpc::types::TransactionRequest::default()
                .to(pair_address)
                .from(deployer)
                .input(mint_calldata.into()),
        )
        .await?
        .get_receipt()
        .await?;
    Ok(())
}

/// Plant a functional UniswapV2 factory at the canonical mainnet
/// address, deploying with the harness deployer as `feeToSetter` so
/// constructor storage is properly initialized, then copying the
/// initialized runtime to the canonical address via `anvil_setCode`.
pub async fn plant_uniswap_v2_factory(roles: &Roles) -> eyre::Result<Address> {
    let existing =
        roles.provider().get_code_at(MAINNET_UNISWAP_V2_FACTORY).await?;
    if !existing.is_empty() {
        return Ok(MAINNET_UNISWAP_V2_FACTORY);
    }

    let artifact: serde_json::Value = serde_json::from_str(include_str!(
        "../abi/damn-vulnerable-defi/UniswapV2Factory.json"
    ))?;
    let hex = artifact["bytecode"]
        .as_str()
        .ok_or_else(|| eyre::eyre!("factory artifact missing bytecode"))?;
    let code: Bytes = hex.parse()?;

    // The factory constructor takes feeToSetter; append the deployer
    // address (abi-encoded) to the creation code.
    let calldata =
        [code.as_ref(), &roles.deployer.address().abi_encode()].concat();

    // Deploy the factory normally — the constructor stores feeToSetter
    // (slot 1) — then copy the initialized runtime to the canonical
    // address via anvil_setCode so the storage-backed init comes from a
    // real deploy, not a hand-written slot write.
    let receipt = roles
        .provider()
        .send_transaction(
            alloy::rpc::types::TransactionRequest::default()
                .create()
                .with_from(roles.deployer.address())
                .with_input(calldata),
        )
        .await?
        .get_receipt()
        .await?;
    let deployed = receipt
        .contract_address()
        .ok_or_else(|| eyre::eyre!("factory deploy produced no address"))?;

    let runtime = roles.provider().get_code_at(deployed).await?;
    set_code(roles, MAINNET_UNISWAP_V2_FACTORY, runtime).await?;

    // anvil_setCode copies only code; the constructor-initialized
    // storage (feeToSetter at slot 1) must be copied explicitly so the
    // canonical address reflects a real deploy.
    let fee_to_setter = roles
        .provider()
        .get_storage_at(deployed, alloy::primitives::U256::from(1))
        .await?;
    set_storage(
        roles,
        MAINNET_UNISWAP_V2_FACTORY,
        U256::from(1),
        B256::from(fee_to_setter.to_be_bytes()),
    )
    .await?;
    Ok(MAINNET_UNISWAP_V2_FACTORY)
}

/// Steer the deployer nonce so the next contract's FIRST nested CREATE
/// (a token created inside its constructor) lands at an address below
/// `bound`. Needed for setups whose pair math requires the challenge
/// token to sort before a planted canonical token (e.g. broker: the
/// UniswapV2 pair's `rate()` uses reserve0/reserve1 integer division,
/// so the token must be token0).
pub async fn steer_deployer_below(
    roles: &Roles,
    bound: Address,
) -> eyre::Result<()> {
    let deployer = roles.deployer.address();
    let mut nonce = roles.provider().get_transaction_count(deployer).await?;
    let mut token_addr = deployer.create(nonce).create(1);
    while token_addr >= bound {
        nonce += 1;
        token_addr = deployer.create(nonce).create(1);
    }
    // Advance with real (empty) self-transactions rather than
    // anvil_setNonce: the latter desyncs the wallet filler's cached
    // nonce and the next send fails with `nonce too low`.
    while roles.provider().get_transaction_count(deployer).await? < nonce {
        roles
            .provider()
            .send_transaction(
                alloy::rpc::types::TransactionRequest::default()
                    .from(deployer)
                    .to(deployer),
            )
            .await?
            .get_receipt()
            .await?;
    }
    Ok(())
}

/// Unlock an arbitrary account (anvil `impersonateAccount`).
pub async fn impersonate(roles: &Roles, addr: Address) -> eyre::Result<()> {
    roles
        .provider()
        .client()
        .request::<_, ()>("anvil_impersonateAccount", (addr,))
        .await?;
    Ok(())
}

/// Send an unlocked `eth_sendTransaction` as `from` (must be impersonated
/// or prefunded by anvil). Used for fork whales and synthetic users.
pub async fn send_as(
    roles: &Roles,
    from: Address,
    to: Address,
    data: alloy::primitives::Bytes,
) -> eyre::Result<()> {
    let tx = serde_json::json!({
        "from": from,
        "to": to,
        "data": data,
    });
    let tx_hash: B256 =
        roles.provider().client().request("eth_sendTransaction", (tx,)).await?;
    for _ in 0..60 {
        let mined: Option<serde_json::Value> = roles
            .provider()
            .client()
            .request("eth_getTransactionReceipt", (tx_hash,))
            .await?;
        if let Some(receipt) = mined {
            let ok = receipt["status"].as_str() == Some("0x1");
            if !ok {
                return Err(eyre::eyre!("impersonated tx {tx_hash} reverted"));
            }
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
    Err(eyre::eyre!("impersonated tx {tx_hash} never mined"))
}

/// Broadcast a pre-signed raw transaction (anvil `sendRawTransaction`),
/// waiting for its receipt.
pub async fn broadcast_raw(roles: &Roles, raw_tx: &str) -> eyre::Result<B256> {
    let tx_hash: B256 = roles
        .provider()
        .client()
        .request("eth_sendRawTransaction", (raw_tx,))
        .await?;
    for _ in 0..60 {
        let receipt: Option<serde_json::Value> = roles
            .provider()
            .client()
            .request("eth_getTransactionReceipt", (tx_hash,))
            .await?;
        if let Some(receipt) = receipt {
            if receipt["status"].as_str() != Some("0x1") {
                return Err(eyre::eyre!("raw tx {tx_hash} reverted"));
            }
            return Ok(tx_hash);
        }
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
    Err(eyre::eyre!("raw tx {tx_hash} never mined"))
}
