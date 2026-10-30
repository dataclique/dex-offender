//! Deploy Uniswap bytecode the upstream dvd way: raw creation txs from
//! the standard-json builds the CTF itself ships under
//! `lib/damn-vulnerable-defi/builds/uniswap`, exactly like the upstream
//! tests' `deployCode` calls.
use crate::Roles;
use alloy::network::TransactionBuilder;
use alloy::sol_types::SolValue;
use alloy::{
    network::ReceiptResponse,
    primitives::{Address, Bytes},
    providers::Provider,
    rpc::types::TransactionRequest,
};

fn standard_json_creation_code(artifact: &str) -> eyre::Result<Bytes> {
    let artifact: serde_json::Value = serde_json::from_str(artifact)?;
    let hex =
        artifact["evm"]["bytecode"]["object"].as_str().ok_or_else(|| {
            eyre::eyre!("standard-json artifact missing evm.bytecode.object")
        })?;
    Ok(hex.parse()?)
}

async fn deploy_code(
    roles: &Roles,
    code: Bytes,
    encoded_ctor_args: &[Vec<u8>],
) -> eyre::Result<Address> {
    let mut input = code.as_ref().to_vec();
    for arg in encoded_ctor_args {
        input.extend_from_slice(arg);
    }
    let receipt = roles
        .provider()
        .send_transaction(
            TransactionRequest::default()
                .create()
                .with_input(input)
                .from(roles.deployer.address()),
        )
        .await?
        .get_receipt()
        .await?;
    receipt.contract_address().ok_or_else(|| {
        eyre::eyre!("creation transaction produced no contract address")
    })
}

const V1_FACTORY_BUILD: &str = include_str!(
    "../../lib/damn-vulnerable-defi/builds/uniswap/UniswapV1Factory.json"
);
const V1_EXCHANGE_BUILD: &str = include_str!(
    "../../lib/damn-vulnerable-defi/builds/uniswap/UniswapV1Exchange.json"
);
const V2_FACTORY_BUILD: &str = include_str!(
    "../../lib/damn-vulnerable-defi/builds/uniswap/UniswapV2Factory.json"
);
const V2_ROUTER02_BUILD: &str = include_str!(
    "../../lib/damn-vulnerable-defi/builds/uniswap/UniswapV2Router02.json"
);

/// Deploy a Uniswap V1 factory (with a fresh template exchange) and the
/// template itself, as dvd's Puppet test does with deployCode.
pub async fn deploy_v1(roles: &Roles) -> eyre::Result<(Address, Address)> {
    let template = deploy_code(
        roles,
        standard_json_creation_code(V1_EXCHANGE_BUILD)?,
        &[],
    )
    .await?;
    let factory = deploy_code(
        roles,
        standard_json_creation_code(V1_FACTORY_BUILD)?,
        &[template.abi_encode()],
    )
    .await?;
    Ok((factory, template))
}

/// Deploy a Uniswap V2 factory + Router02 pair (Router02 ctor takes
/// factory and WETH).
pub async fn deploy_v2(
    roles: &Roles,
    weth: Address,
) -> eyre::Result<(Address, Address)> {
    let factory = deploy_code(
        roles,
        standard_json_creation_code(V2_FACTORY_BUILD)?,
        &[Address::ZERO.abi_encode()],
    )
    .await?;
    let router = deploy_code(
        roles,
        standard_json_creation_code(V2_ROUTER02_BUILD)?,
        &[factory.abi_encode(), weth.abi_encode()],
    )
    .await?;
    Ok((factory, router))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_json_builds_embed_creation_code() -> eyre::Result<()> {
        for build in [
            V1_FACTORY_BUILD,
            V1_EXCHANGE_BUILD,
            V2_FACTORY_BUILD,
            V2_ROUTER02_BUILD,
        ] {
            let code = standard_json_creation_code(build)?;
            assert!(
                !code.is_empty(),
                "empty creation code in standard-json build"
            );
        }
        Ok(())
    }
}
