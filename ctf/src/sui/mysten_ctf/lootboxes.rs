//! mysten-ctf `lootboxes` — paid randomized lootbox level.
//!
//! `open_lootbox` burns a 12 USDC payment (USDC has six decimals) and
//! randomly mints a `MaybeFlag` that may hold a `ctf::flag::Flag`. The
//! harness deploys a local USDC package (from the same pinned circlefin
//! source the fixture builds), rewrites the lootbox module's compiled
//! USDC reference to that local package, then publishes the level. The
//! check observes the solved state: the offender owns a flag object.
//! How to open boxes until a flag appears is the level's answer and
//! stays out of the harness.

use crate::sui::{PublishOutcome, SuiLocalnet, SuiRoles};
use sui_sdk_types::Address;

/// Compiled bytecode (built via ctf/fixtures/mysten_ctf, see
/// scripts/build-sui-levels.nu).
pub const USDC_MODULE: &[u8] = include_bytes!(
    "../../../../ctf/fixtures/mysten_ctf/build/mysten_ctf/bytecode_modules/dependencies/usdc/usdc.mv"
);
const LEVEL_MODULES: [&[u8]; 2] = [
    include_bytes!(
        "../../../../ctf/fixtures/mysten_ctf/build/mysten_ctf/bytecode_modules/dependencies/ctf/lootboxes.mv"
    ),
    include_bytes!(
        "../../../../ctf/fixtures/mysten_ctf/build/mysten_ctf/bytecode_modules/dependencies/ctf/flag.mv"
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

pub async fn set_up(
    localnet: &SuiLocalnet,
    roles: &SuiRoles,
) -> eyre::Result<Target> {
    // 1) publish a local USDC package; its compiled-in original id
    //    tells us the address the level bytecode references
    let usdc_original = crate::sui::original_self_address(USDC_MODULE)?;
    let usdc_outcome: PublishOutcome = localnet
        .publish(
            &roles.deployer,
            vec![crate::sui::zero_self_address(USDC_MODULE)?],
            vec![
                Address::from_static(
                    "0x0000000000000000000000000000000000000000000000000000000000000001",
                ),
                Address::from_static(
                    "0x0000000000000000000000000000000000000000000000000000000000000002",
                ),
                Address::from_static(
                    "0x0000000000000000000000000000000000000000000000000000000000000003",
                ),
            ],
        )
        .await?;
    let usdc = Address::from_hex(&usdc_outcome.package)?;

    // 2) rewrite the level modules' USDC reference to the local package
    //    and zero their self addresses for fresh publish
    let mut modules = Vec::with_capacity(LEVEL_MODULES.len());
    for module in LEVEL_MODULES {
        let rewritten =
            crate::sui::rewrite_address(module, usdc_original, usdc)?;
        modules.push(crate::sui::zero_self_address(&rewritten)?);
    }

    // 3) publish the level package
    let outcome: PublishOutcome = localnet
        .publish(
            &roles.deployer,
            modules,
            vec![
                Address::from_static(
                    "0x0000000000000000000000000000000000000000000000000000000000000001",
                ),
                Address::from_static(
                    "0x0000000000000000000000000000000000000000000000000000000000000002",
                ),
                Address::from_static(
                    "0x0000000000000000000000000000000000000000000000000000000000000003",
                ),
                usdc,
            ],
        )
        .await?;
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
