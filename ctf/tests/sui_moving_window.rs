//! mysten-ctf sabotage_arena + moving_window red-step tests: the
//! moving_window module API does not exist yet (compile failure), and
//! arena integration stays green.

#[test]
fn fresh_state_parses_ten() {
    // arena regression must stay green while moving_window lands
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

use sui_sdk_types::Address;

#[test]
fn flag_type_string_formats_with_package_address() {
    // moving_window check builds `0x<pkg>::flag::Flag` from the target
    let pkg = Address::from_static(
        "0x0000000000000000000000000000000000000000000000000000000000000001",
    );
    let t = ctf::sui::mysten_ctf::moving_window::flag_type(&pkg);
    assert!(t.ends_with("::flag::Flag"), "got {t}");
    assert!(t.starts_with("0x"), "got {t}");
    assert!(t.contains(&pkg.to_string()), "got {t}");
}

#[tokio::test]
async fn deploys_moving_window_unsolved() {
    if std::env::var("SUI_LOCALNET_TEST").is_err() {
        eprintln!(
            "SUI_LOCALNET_TEST not set; skipping moving_window live test"
        );
        return;
    }
    use ctf::sui::SuiLocalnet;
    use ctf::sui::mysten_ctf::moving_window;

    let (localnet, roles) =
        SuiLocalnet::start_with_roles().await.expect("localnet with roles");
    let result = {
        async {
            let target = moving_window::set_up(&localnet, &roles).await?;
            println!("moving_window package {}", target.package);
            let solved =
                moving_window::check(&target, &roles, &localnet).await?;
            Ok::<_, eyre::Report>(solved)
        }
        .await
    };
    let scratch = localnet.scratch_dir().to_path_buf();
    localnet.teardown().await.expect("teardown");

    let solved = result.expect("moving_window setup+check");
    assert!(!solved, "fresh level must be unsolved (no Flag owned)");
    assert!(!scratch.exists(), "scratch removed");
}
