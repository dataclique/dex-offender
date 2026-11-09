//! mysten-ctf `lootboxes` — paid randomized lootbox level.
//!
//! `open_lootbox` burns a 12 USDC payment (USDC has six decimals) and
//! randomly mints a `MaybeFlag` that may hold a `ctf::flag::Flag`. The
//! harness deploys the pinned USDC dependency chain locally
//! (sui_extensions → stablecoin → usdc, each rewritten to the fresh
//! local package ids), points the level modules that reference USDC at
//! the local package, then publishes the level. The check observes the
//! solved state: the offender owns a flag object.
//! How to open boxes until a flag appears is the level's answer and
//! stays out of the harness.

use crate::sui::{PublishOutcome, SuiLocalnet, SuiRoles};
use sui_sdk_types::Address;

/// Compiled bytecode (built via ctf/fixtures/mysten_ctf, see
/// scripts/build-sui-levels.nu).
pub const USDC_MODULE: &[u8] = include_bytes!(
    "../../../fixtures/mysten_ctf/build/mysten_ctf/bytecode_modules/dependencies/usdc/usdc.mv"
);
const SUI_EXTENSIONS_MODULES: [&[u8]; 2] = [
    include_bytes!(
        "../../../fixtures/mysten_ctf/build/mysten_ctf/bytecode_modules/dependencies/sui_extensions/upgrade_service.mv"
    ),
    include_bytes!(
        "../../../fixtures/mysten_ctf/build/mysten_ctf/bytecode_modules/dependencies/sui_extensions/two_step_role.mv"
    ),
];
const STABLECOIN_MODULES: [&[u8]; 6] = [
    include_bytes!(
        "../../../fixtures/mysten_ctf/build/mysten_ctf/bytecode_modules/dependencies/stablecoin/entry.mv"
    ),
    include_bytes!(
        "../../../fixtures/mysten_ctf/build/mysten_ctf/bytecode_modules/dependencies/stablecoin/version_control.mv"
    ),
    include_bytes!(
        "../../../fixtures/mysten_ctf/build/mysten_ctf/bytecode_modules/dependencies/stablecoin/treasury.mv"
    ),
    include_bytes!(
        "../../../fixtures/mysten_ctf/build/mysten_ctf/bytecode_modules/dependencies/stablecoin/stablecoin.mv"
    ),
    include_bytes!(
        "../../../fixtures/mysten_ctf/build/mysten_ctf/bytecode_modules/dependencies/stablecoin/roles.mv"
    ),
    include_bytes!(
        "../../../fixtures/mysten_ctf/build/mysten_ctf/bytecode_modules/dependencies/stablecoin/mint_allowance.mv"
    ),
];
const LEVEL_MODULES: [&[u8]; 2] = [
    include_bytes!(
        "../../../fixtures/mysten_ctf/build/mysten_ctf/bytecode_modules/dependencies/ctf/lootboxes.mv"
    ),
    include_bytes!(
        "../../../fixtures/mysten_ctf/build/mysten_ctf/bytecode_modules/dependencies/ctf/flag.mv"
    ),
];

const FRAMEWORK_DEPS: [Address; 3] = [
    Address::from_static(
        "0x0000000000000000000000000000000000000000000000000000000000000001",
    ),
    Address::from_static(
        "0x0000000000000000000000000000000000000000000000000000000000000002",
    ),
    Address::from_static(
        "0x0000000000000000000000000000000000000000000000000000000000000003",
    ),
];

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub package: Address,
    /// The local USDC package the level's payment coin references.
    pub usdc: Address,
}

/// The full Move type of the minted flag, `0x<pkg>::flag::Flag`.
pub fn flag_type(package: &Address) -> String {
    format!("{package}::flag::Flag")
}

/// Test accessor for the compiled stablecoin entry module.
pub fn stablecoin_entry_for_test() -> &'static [u8] {
    STABLECOIN_MODULES[0]
}

/// Test accessor: every stablecoin module's referenced identifiers.
/// Deserialization failures propagate as typed errors.
pub fn stablecoin_identifiers_for_test()
-> eyre::Result<Vec<Vec<sui_sdk_types::Address>>> {
    STABLECOIN_MODULES
        .iter()
        .map(|m| crate::sui::address_identifiers(m))
        .collect()
}

/// Test accessor: every sui_extensions module's referenced identifiers.
/// Deserialization failures propagate as typed errors.
pub fn sui_extensions_identifiers_for_test()
-> eyre::Result<Vec<Vec<sui_sdk_types::Address>>> {
    SUI_EXTENSIONS_MODULES
        .iter()
        .map(|m| crate::sui::address_identifiers(m))
        .collect()
}

pub async fn set_up(
    localnet: &SuiLocalnet,
    roles: &SuiRoles,
) -> eyre::Result<Target> {
    // 1) publish the pinned dependency chain exactly as the sources
    //    depend on it: sui_extensions → stablecoin → usdc. Each
    //    dependent module's references to its dependency's original
    //    (testnet) package id are rewritten to the fresh local id;
    //    every self address is zeroed for raw publish.
    let sui_ext_original =
        crate::sui::original_self_address(SUI_EXTENSIONS_MODULES[0])?;
    let sui_ext_modules = SUI_EXTENSIONS_MODULES
        .iter()
        .map(|m| crate::sui::zero_self_address(m))
        .collect::<eyre::Result<Vec<_>>>()?;
    let sui_ext_outcome: PublishOutcome = localnet
        .publish(&roles.deployer, sui_ext_modules, FRAMEWORK_DEPS.to_vec())
        .await?;
    let sui_ext = Address::from_hex(&sui_ext_outcome.package)?;

    let stablecoin_original =
        crate::sui::original_self_address(STABLECOIN_MODULES[0])?;
    let stablecoin_modules = STABLECOIN_MODULES
        .iter()
        .map(|m| {
            // only modules that import sui_extensions reference its id
            let prepared = if crate::sui::address_identifiers(m)?
                .contains(&sui_ext_original)
            {
                crate::sui::rewrite_address(m, sui_ext_original, sui_ext)?
            } else {
                m.to_vec()
            };
            crate::sui::zero_self_address(&prepared)
        })
        .collect::<eyre::Result<Vec<_>>>()?;
    let stablecoin_deps =
        FRAMEWORK_DEPS.iter().copied().chain([sui_ext]).collect();
    let stablecoin_outcome: PublishOutcome = localnet
        .publish(&roles.deployer, stablecoin_modules, stablecoin_deps)
        .await?;
    let stablecoin = Address::from_hex(&stablecoin_outcome.package)?;

    // 2) publish the local USDC package, pointing it at the local
    //    sui_extensions and stablecoin packages
    let usdc_original = crate::sui::original_self_address(USDC_MODULE)?;
    let usdc_prepared = {
        let rewritten = crate::sui::rewrite_address(
            USDC_MODULE,
            sui_ext_original,
            sui_ext,
        )?;
        let rewritten = crate::sui::rewrite_address(
            &rewritten,
            stablecoin_original,
            stablecoin,
        )?;
        crate::sui::zero_self_address(&rewritten)?
    };
    let usdc_deps =
        FRAMEWORK_DEPS.iter().copied().chain([sui_ext, stablecoin]).collect();
    let usdc_outcome: PublishOutcome = localnet
        .publish(&roles.deployer, vec![usdc_prepared], usdc_deps)
        .await?;
    let usdc = Address::from_hex(&usdc_outcome.package)?;

    // 3) point the level modules at the local USDC package: only the
    //    modules that actually reference it (lootboxes) get the
    //    identifier rewritten — the same-package flag module does not.
    //    Every module's self address is then zeroed for fresh publish.
    let mut modules = Vec::with_capacity(LEVEL_MODULES.len());
    for module in LEVEL_MODULES {
        let referenced =
            crate::sui::address_identifiers(module)?.contains(&usdc_original);
        let prepared = if referenced {
            crate::sui::rewrite_address(module, usdc_original, usdc)?
        } else {
            module.to_vec()
        };
        modules.push(crate::sui::zero_self_address(&prepared)?);
    }

    // 4) publish the level package
    let level_deps = FRAMEWORK_DEPS.iter().copied().chain([usdc]).collect();
    let outcome: PublishOutcome =
        localnet.publish(&roles.deployer, modules, level_deps).await?;
    Ok(Target { package: Address::from_hex(&outcome.package)?, usdc })
}

/// Solved when the offender owns a flag object of this package's type.
pub async fn check(
    target: &Target,
    roles: &SuiRoles,
    localnet: &SuiLocalnet,
) -> eyre::Result<bool> {
    use sui_rpc::field::{FieldMask, FieldMaskUtil};
    let mut client = localnet.client();
    let mut state = client.state_client();
    let mut request =
        sui_rpc::proto::sui::rpc::v2::ListOwnedObjectsRequest::const_default();
    request.owner = Some(roles.offender.address.to_string());
    request.object_type = Some(flag_type(&target.package));
    request.read_mask =
        Some(FieldMask::from_paths(["object_type".to_string()]));
    let response = state.list_owned_objects(request).await?.into_inner();
    Ok(!response.objects.is_empty())
}
