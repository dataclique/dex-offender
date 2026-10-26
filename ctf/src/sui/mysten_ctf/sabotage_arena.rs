//! mysten-ctf `sabotage_arena` — shared Arena PvP level.
//!
//! Module `init` shares an `Arena` (10 flags, shield threshold 12,
//! cooldown 600s, deadline) and transfers an `AdminCap` to the
//! publisher. Players register/build shields/attack; `claim_flag`
//! requires `shield >= 12` and decrements `flags_remaining`. The
//! harness publishes the pinned shared package (built through the
//! mysten_ctf fixture — see ctf/fixtures/mysten_ctf) and checks the
//! observable solved state: `flags_remaining < 10`. How to reach the
//! shield threshold within the cooldown is the level's answer and stays
//! out of the harness. Local actors only; no external players.

use crate::sui::{PublishOutcome, SuiLocalnet, SuiRoles};
use std::time::{Duration, Instant};
use sui_rpc::proto::sui::rpc::v2::owner::OwnerKind;
use sui_sdk_types::Address;

/// Compiled bytecode for this level's module subset (built via
/// ctf/fixtures/mysten_ctf, see scripts/build-sui-levels.nu): the
/// arena plus its `ctf::flag` dependency. The remaining shared-package
/// modules (merchant needs an on-chain USDC package) deploy with their
/// own level units.
const LEVEL_MODULES: [&[u8]; 2] = [
    include_bytes!(
        "../../../../ctf/fixtures/mysten_ctf/build/mysten_ctf/bytecode_modules/dependencies/ctf/sabotage_arena.mv"
    ),
    include_bytes!(
        "../../../../ctf/fixtures/mysten_ctf/build/mysten_ctf/bytecode_modules/dependencies/ctf/flag.mv"
    ),
];

/// Initial flag count minted by `init` (from the pinned source).
pub const INITIAL_FLAGS: u8 = 10;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub package: Address,
    /// The shared Arena object created by module `init`.
    pub arena: Address,
}

/// Parse `flags_remaining` out of the shared Arena's BCS layout
/// (grounded in the pinned struct definition):
/// `UID (32) ‖ Table.players { UID (32), size u64 (8) } ‖ u8 ‖ u64 ‖
/// u64 ‖ u64`. Malformed or truncated input is a typed error, never a
/// panic.
pub fn parse_arena_flags_remaining(bcs: &[u8]) -> eyre::Result<u8> {
    const FLAGS_OFFSET: usize = 32 + 32 + 8;
    const MIN_LEN: usize = FLAGS_OFFSET + 1 + 8 + 8 + 8;
    if bcs.len() < MIN_LEN {
        return Err(eyre::eyre!(
            "malformed Arena bcs: len {} < {} (layout drift?)",
            bcs.len(),
            MIN_LEN
        ));
    }
    Ok(bcs[FLAGS_OFFSET])
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
    let package = Address::from_hex(&outcome.package)?;
    // init shares the Arena: created + shared ownership
    let arena =
        outcome.created_with_owner(OwnerKind::Shared as i32).ok_or_else(
            || eyre::eyre!("sabotage_arena init created no shared Arena"),
        )?;
    let arena = Address::from_hex(arena)?;

    // wait until the shared object is readable (checkpoint lag)
    let client = localnet.client();
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let mut ledger = client.clone().ledger_client();
        let mut request =
            sui_rpc::proto::sui::rpc::v2::GetObjectRequest::const_default();
        request.object_id = Some(arena.to_string());
        if ledger.get_object(request).await.is_ok() {
            break;
        }
        if Instant::now() > deadline {
            return Err(eyre::eyre!("shared Arena never became readable"));
        }
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }

    Ok(Target { package, arena })
}

/// Read the shared Arena's `flags_remaining` via gRPC. A fresh level
/// reads `INITIAL_FLAGS` (unsolved); each successful claim decrements.
pub async fn check(
    target: &Target,
    localnet: &SuiLocalnet,
) -> eyre::Result<bool> {
    use sui_rpc::field::{FieldMask, FieldMaskUtil};
    let mut client = localnet.client();
    let mut ledger = client.ledger_client();
    let mut request =
        sui_rpc::proto::sui::rpc::v2::GetObjectRequest::const_default();
    request.object_id = Some(target.arena.to_string());
    request.read_mask = Some(FieldMask::from_paths(["bcs.value".to_string()]));
    let raw = ledger
        .get_object(request)
        .await?
        .into_inner()
        .object
        .ok_or_else(|| eyre::eyre!("Arena object missing"))?
        .bcs
        .and_then(|b| b.value)
        .ok_or_else(|| eyre::eyre!("Arena object missing bcs payload"))?;
    // gRPC payload is the BCS sui_sdk_types::Object envelope; the Move
    // struct value lives in its struct contents
    let envelope: sui_sdk_types::Object = bcs::from_bytes(&raw)
        .map_err(|e| eyre::eyre!("deserialize object envelope: {e:?}"))?;
    let contents = match envelope.data() {
        sui_sdk_types::ObjectData::Struct(m) => m.contents().to_vec(),
        sui_sdk_types::ObjectData::Package(_) => {
            return Err(eyre::eyre!("Arena object is a package"));
        }
    };
    Ok(parse_arena_flags_remaining(&contents)? < INITIAL_FLAGS)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn arena_bcs(flags_remaining: u8) -> Vec<u8> {
        let mut out = vec![0u8; 32]; // Arena id UID
        out.extend_from_slice(&[0u8; 32]); // Table id UID
        out.extend_from_slice(&0u64.to_le_bytes()); // Table size
        out.push(flags_remaining);
        out.extend_from_slice(&12u64.to_le_bytes()); // shield_threshold
        out.extend_from_slice(&600_000u64.to_le_bytes()); // cooldown_ms
        out.extend_from_slice(&1_772_974_800_000u64.to_le_bytes()); // deadline
        out
    }

    #[test]
    fn parses_fresh_state_as_ten() {
        assert_eq!(parse_arena_flags_remaining(&arena_bcs(10)).unwrap(), 10);
    }

    #[test]
    fn parses_claimed_state() {
        assert_eq!(parse_arena_flags_remaining(&arena_bcs(9)).unwrap(), 9);
    }

    #[test]
    fn truncated_bcs_is_a_typed_error() {
        let err =
            parse_arena_flags_remaining(&[0u8; 40]).expect_err("must fail");
        assert!(err.to_string().contains("malformed Arena"), "got: {err}");
    }

    #[test]
    fn layout_total_is_ninety_seven_bytes() {
        assert_eq!(arena_bcs(10).len(), 97, "documented layout length");
    }
}
