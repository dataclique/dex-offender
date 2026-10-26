//! lets-ctf `check_in` — shared FlagString challenge.
//!
//! The module `init` shares a `FlagString { str, ture_num }`; `get_flag`
//! verifies a sha3 preimage over the current string plus a github id and,
//! on success, rotates the string randomly and increments `ture_num`.
//! The harness deploys the pinned package (built through the
//! lets_ctf_check_in fixture — see ctf/fixtures/lets_ctf_check_in) and
//! checks the observable solved state: `ture_num > 0`. The preimage is
//! the level's answer and stays out of the harness.

use crate::sui::{PublishOutcome, SuiLocalnet, SuiRoles};
use std::time::{Duration, Instant};
use sui_rpc::proto::sui::rpc::v2::owner::OwnerKind;
use sui_sdk_types::Address;

/// Compiled check_in bytecode (build the pinned package directly:
/// `sui move build` in lib/lets-ctf/src/01_check_in/check_in with the
/// gitee→github git URL rewrite — see scripts/build-sui-levels.nu).
const CHECK_IN_MODULE: &[u8] = include_bytes!(
    "../../../../lib/lets-ctf/src/01_check_in/check_in/build/check_in/bytecode_modules/check_in.mv"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub package: Address,
    /// The shared FlagString object created by module `init`.
    pub flag_string: Address,
}

/// Parse `ture_num` out of the shared FlagString's BCS layout:
/// `UID (32-byte address) ‖ String (uleb128 len + bytes) ‖ u64`.
/// Malformed or truncated input is a typed error, never a panic.
pub fn parse_flag_string_ture_num(bcs: &[u8]) -> eyre::Result<u64> {
    let bad = |what: &str| {
        eyre::eyre!("malformed FlagString bcs: {what} (len {})", bcs.len())
    };
    if bcs.len() < 32 {
        return Err(bad("shorter than a UID"));
    }
    let rest = &bcs[32..];
    // uleb128 string length (at most 4 bytes for sane strings)
    let mut len = 0usize;
    let mut shift = 0u32;
    let mut consumed = 0usize;
    loop {
        let byte =
            *rest.get(consumed).ok_or_else(|| bad("truncated in length"))?;
        consumed += 1;
        len |= ((byte & 0x7f) as usize) << shift;
        if byte & 0x80 == 0 {
            break;
        }
        shift += 7;
        if consumed > 4 {
            return Err(bad("length varint too long"));
        }
    }
    let rest =
        rest.get(consumed..).ok_or_else(|| bad("truncated after length"))?;
    let rest =
        rest.get(len..).ok_or_else(|| bad("truncated in string body"))?;
    let num =
        rest.first_chunk::<8>().ok_or_else(|| bad("missing ture_num u64"))?;
    Ok(u64::from_le_bytes(*num))
}

pub async fn set_up(
    localnet: &SuiLocalnet,
    roles: &SuiRoles,
) -> eyre::Result<Target> {
    // the upstream package pins its testnet original-id in
    // Published.toml; raw publish requires self-address 0x0
    let module = crate::sui::zero_self_address(CHECK_IN_MODULE)?;
    let outcome: PublishOutcome = localnet
        .publish(
            &roles.deployer,
            vec![module],
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
    let package = Address::from_hex(&outcome.package)?;
    // init shares the FlagString: created + shared ownership
    let flag_string =
        outcome.created_with_owner(OwnerKind::Shared as i32).ok_or_else(
            || eyre::eyre!("check_in init created no shared FlagString"),
        )?;
    let flag_string = Address::from_hex(flag_string)?;

    // wait until the shared object is readable (checkpoint lag)
    let client = localnet.client();
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let mut ledger = client.clone().ledger_client();
        let mut request =
            sui_rpc::proto::sui::rpc::v2::GetObjectRequest::const_default();
        request.object_id = Some(flag_string.to_string());
        if ledger.get_object(request).await.is_ok() {
            break;
        }
        if Instant::now() > deadline {
            return Err(eyre::eyre!("shared FlagString never became readable"));
        }
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }

    Ok(Target { package, flag_string })
}

/// Read the shared FlagString's `ture_num` via gRPC. A fresh level is
/// unsolved (0); any successful `get_flag` increments it.
pub async fn check(
    target: &Target,
    localnet: &SuiLocalnet,
) -> eyre::Result<bool> {
    let mut client = localnet.client();
    let mut ledger = client.ledger_client();
    let mut request =
        sui_rpc::proto::sui::rpc::v2::GetObjectRequest::const_default();
    request.object_id = Some(target.flag_string.to_string());
    use sui_rpc::field::{FieldMask, FieldMaskUtil};
    request.read_mask = Some(FieldMask::from_paths(["bcs.value".to_string()]));
    let object = ledger
        .get_object(request)
        .await?
        .into_inner()
        .object
        .ok_or_else(|| eyre::eyre!("FlagString object missing"))?;
    let raw = object
        .bcs
        .and_then(|b| b.value)
        .ok_or_else(|| eyre::eyre!("FlagString object missing bcs payload"))?;
    // the gRPC payload is the BCS sui_sdk_types::Object envelope; the
    // Move struct value lives in its struct contents
    let envelope: sui_sdk_types::Object = bcs::from_bytes(&raw)
        .map_err(|e| eyre::eyre!("deserialize object envelope: {e:?}"))?;
    let contents = match envelope.data() {
        sui_sdk_types::ObjectData::Struct(m) => m.contents().to_vec(),
        sui_sdk_types::ObjectData::Package(_) => {
            return Err(eyre::eyre!("FlagString object is a package"));
        }
    };
    Ok(parse_flag_string_ture_num(&contents)? > 0)
}

// The chain-agnostic Level/Exploit re-exports live in crate::sui; Sui
// level modules use the Sui-specific set_up/check pair above.

#[cfg(test)]
mod tests {
    use super::*;

    fn flag_string_bcs(str_bytes: &[u8], ture_num: u64) -> Vec<u8> {
        let mut out = vec![0u8; 32]; // UID address
        out.push(str_bytes.len() as u8); // uleb128 length (short strings)
        out.extend_from_slice(str_bytes);
        out.extend_from_slice(&ture_num.to_le_bytes());
        out
    }

    #[test]
    fn embedded_bytecode_self_address_is_zeroed() {
        let rewritten = crate::sui::zero_self_address(CHECK_IN_MODULE)
            .expect("check_in bytecode rewrites");
        let module =
            move_binary_format::CompiledModule::deserialize_with_defaults(
                &rewritten,
            )
            .expect("rewritten module deserializes");
        use move_binary_format::internals::ModuleIndex;
        let addr_idx = module.module_handles
            [module.self_handle_idx().into_index()]
        .address
        .into_index();
        assert_eq!(
            module.address_identifiers[addr_idx],
            move_core_types::account_address::AccountAddress::ZERO,
            "self address must be zero for fresh publish"
        );
    }

    #[test]
    fn parses_zero_for_fresh_state() {
        let bcs = flag_string_bcs(b"LetsMoveCTF", 0);
        assert_eq!(parse_flag_string_ture_num(&bcs).unwrap(), 0);
    }

    #[test]
    fn parses_incremented_state() {
        let bcs = flag_string_bcs(b"rotated-secret", 3);
        assert_eq!(parse_flag_string_ture_num(&bcs).unwrap(), 3);
    }

    #[test]
    fn truncated_bcs_is_a_typed_error() {
        let err =
            parse_flag_string_ture_num(&[0u8; 10]).expect_err("must fail");
        assert!(err.to_string().contains("malformed FlagString"), "got: {err}");
    }

    #[test]
    fn missing_tail_is_a_typed_error() {
        let mut bcs = flag_string_bcs(b"LetsMoveCTF", 7);
        bcs.truncate(bcs.len() - 3); // cut into the u64
        let err = parse_flag_string_ture_num(&bcs).expect_err("must fail");
        assert!(err.to_string().contains("malformed FlagString"), "got: {err}");
    }
}
