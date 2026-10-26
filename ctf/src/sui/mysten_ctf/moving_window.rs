//! mysten-ctf `moving_window` — time-gated flag level.
//!
//! `extract_flag(clock, ctx)` mints a `ctf::flag::Flag` only during
//! limited windows of each hour. The harness publishes the module
//! subset (moving_window + flag) and checks the observable solved
//! state: the offender owns a `0x<pkg>::flag::Flag` object. When to
//! call is the level's answer and stays out of the harness.

use crate::sui::{PublishOutcome, SuiLocalnet, SuiRoles};
use sui_sdk_types::Address;

/// Compiled bytecode for this level's module subset (built via
/// ctf/fixtures/mysten_ctf, see scripts/build-sui-levels.nu).
const LEVEL_MODULES: [&[u8]; 2] = [
    include_bytes!(
        "../../../../ctf/fixtures/mysten_ctf/build/mysten_ctf/bytecode_modules/dependencies/ctf/moving_window.mv"
    ),
    include_bytes!(
        "../../../../ctf/fixtures/mysten_ctf/build/mysten_ctf/bytecode_modules/dependencies/ctf/flag.mv"
    ),
];

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub package: Address,
}

/// The full Move type of the minted flag, `0x<pkg>::flag::Flag`.
pub fn flag_type(package: &Address) -> String {
    format!("{package}::flag::Flag")
}

pub async fn set_up(
    localnet: &SuiLocalnet,
    roles: &SuiRoles,
) -> eyre::Result<Target> {
    // the shared package pins its testnet original-id; fresh publish
    // requires self-address 0x0 for every module
    let modules: Vec<Vec<u8>> = LEVEL_MODULES
        .iter()
        .map(|m| crate::sui::zero_self_address(m))
        .collect::<eyre::Result<_>>()?;
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
            ],
        )
        .await?;
    Ok(Target { package: Address::from_hex(&outcome.package)? })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flag_type_matches_package() {
        let pkg = Address::from_static(
            "0x0000000000000000000000000000000000000000000000000000000000000001",
        );
        assert_eq!(flag_type(&pkg), format!("{pkg}::flag::Flag"));
    }
}
