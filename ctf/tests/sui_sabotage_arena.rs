//! mysten-ctf sabotage_arena layout regressions (red step: parser
//! module not yet implemented) + gated live deploy test.

#[test]
fn arena_layout_is_ninety_seven_bytes() {
    // Arena BCS layout (grounded in the pinned struct definition):
    // UID(32) | Table{UID(32), size u64(8)} | u8 | u64 | u64 | u64
    let mut out = vec![0u8; 32];
    out.extend_from_slice(&[0u8; 32]);
    out.extend_from_slice(&0u64.to_le_bytes());
    out.push(10u8);
    out.extend_from_slice(&12u64.to_le_bytes());
    out.extend_from_slice(&600_000u64.to_le_bytes());
    out.extend_from_slice(&1_772_974_800_000u64.to_le_bytes());
    assert_eq!(out.len(), 97, "documented layout length");
}

#[test]
fn fresh_state_parses_ten() {
    let mut out = vec![0u8; 32];
    out.extend_from_slice(&[0u8; 32]);
    out.extend_from_slice(&0u64.to_le_bytes());
    out.push(10u8);
    out.extend_from_slice(&12u64.to_le_bytes());
    out.extend_from_slice(&600_000u64.to_le_bytes());
    out.extend_from_slice(&1_772_974_800_000u64.to_le_bytes());
    assert_eq!(
        ctf::sui::mysten_ctf::sabotage_arena::parse_arena_flags_remaining(&out)
            .unwrap(),
        10
    );
}

#[tokio::test]
async fn deploys_sabotage_arena_unsolved() {
    if std::env::var("SUI_LOCALNET_TEST").is_err() {
        eprintln!(
            "SUI_LOCALNET_TEST not set; skipping sabotage_arena live test"
        );
        return;
    }
    use ctf::sui::SuiLocalnet;
    use ctf::sui::mysten_ctf::sabotage_arena;

    let (localnet, roles) =
        SuiLocalnet::start_with_roles().await.expect("localnet with roles");
    let result = {
        async {
            let target = sabotage_arena::set_up(&localnet, &roles).await?;
            println!(
                "sabotage_arena package {} arena {}",
                target.package, target.arena
            );
            let solved = sabotage_arena::check(&target, &localnet).await?;
            Ok::<_, eyre::Report>(solved)
        }
        .await
    };
    let scratch = localnet.scratch_dir().to_path_buf();
    localnet.teardown().await.expect("teardown");

    let solved = result.expect("sabotage_arena setup+check");
    assert!(!solved, "fresh arena must be unsolved (10 flags)");
    assert!(!scratch.exists(), "scratch removed");
}
