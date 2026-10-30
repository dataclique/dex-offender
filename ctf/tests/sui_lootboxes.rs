#![cfg(feature = "sui")]

//! mysten-ctf lootboxes red-step tests: level API does not exist yet
//! (compile failure is the red step); arena + moving_window stay green.

#[test]
fn arena_regression_stays_green() {
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

#[test]
fn address_rewrite_swaps_only_matching_identifier() {
    let usdc = sui_sdk_types::Address::from_static(
        "0x00000000000000000000000000000000000000000000000000000000000000aa",
    );
    let local = sui_sdk_types::Address::from_static(
        "0x00000000000000000000000000000000000000000000000000000000000000bb",
    );
    // the usdc module's original self address is rewrites' input; verify
    // the core helper rewrites exactly one identifier and keeps others
    let module_bytes = ctf::sui::mysten_ctf::lootboxes::USDC_MODULE;
    let rewritten = ctf::sui::rewrite_address(module_bytes, usdc, local)
        .expect("usdc identifier present");
    let found =
        ctf::sui::address_identifiers(&rewritten).expect("deserializes");
    assert!(found.contains(&local), "local usdc address embedded");
    assert!(!found.contains(&usdc), "original usdc address gone");
}

#[tokio::test]
async fn deploys_lootboxes_unsolved() {
    if std::env::var("SUI_LOCALNET_TEST").is_err() {
        eprintln!("SUI_LOCALNET_TEST not set; skipping lootboxes live test");
        return;
    }
    use ctf::sui::SuiLocalnet;
    use ctf::sui::mysten_ctf::lootboxes;

    let (localnet, roles) =
        SuiLocalnet::start_with_roles().await.expect("localnet with roles");
    let result = {
        async {
            let target = lootboxes::set_up(&localnet, &roles).await?;
            println!(
                "lootboxes package {} usdc {}",
                target.package, target.usdc
            );
            let solved = lootboxes::check(&target, &roles, &localnet).await?;
            Ok::<_, eyre::Report>(solved)
        }
        .await
    };
    let scratch = localnet.scratch_dir().to_path_buf();
    localnet.teardown().await.expect("teardown");

    let solved = result.expect("lootboxes setup+check");
    assert!(!solved, "fresh level must be unsolved (no Flag owned)");
    assert!(!scratch.exists(), "scratch removed");
}
