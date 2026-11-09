#![cfg(feature = "sui")]

//! mysten-ctf lootboxes: address-rewrite regressions (offline) plus the
//! gated live deployment. arena regression stays green.

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
fn rewrite_exercises_the_module_real_referenced_address() {
    let original = ctf::sui::original_self_address(
        ctf::sui::mysten_ctf::lootboxes::USDC_MODULE,
    )
    .expect("usdc module carries its published self address");
    let local = sui_sdk_types::Address::from_static(
        "0x00000000000000000000000000000000000000000000000000000000000000bb",
    );
    let rewritten = ctf::sui::rewrite_address(
        ctf::sui::mysten_ctf::lootboxes::USDC_MODULE,
        original,
        local,
    )
    .expect("referenced address rewrites");
    let found =
        ctf::sui::address_identifiers(&rewritten).expect("deserializes");
    assert!(found.contains(&local), "local usdc address embedded");
    assert!(!found.contains(&original), "original usdc address gone");
}

#[test]
fn rewrite_rejects_an_unreferenced_address() {
    let absent = sui_sdk_types::Address::from_static(
        "0x00000000000000000000000000000000000000000000000000000000000000aa",
    );
    let local = sui_sdk_types::Address::from_static(
        "0x00000000000000000000000000000000000000000000000000000000000000bb",
    );
    let result = ctf::sui::rewrite_address(
        ctf::sui::mysten_ctf::lootboxes::USDC_MODULE,
        absent,
        local,
    );
    assert!(result.is_err(), "unreferenced address must not silently no-op");
}

#[test]
fn print_usdc_referenced_identifiers() {
    let ids = ctf::sui::address_identifiers(
        ctf::sui::mysten_ctf::lootboxes::USDC_MODULE,
    )
    .expect("deserialize");
    for id in &ids {
        println!("usdc references {id}");
    }
    println!(
        "usdc original self {}",
        ctf::sui::original_self_address(
            ctf::sui::mysten_ctf::lootboxes::USDC_MODULE
        )
        .expect("self")
    );
}

#[test]
fn print_stablecoin_referenced_identifiers() {
    let module = ctf::sui::mysten_ctf::lootboxes::stablecoin_entry_for_test();
    let ids = ctf::sui::address_identifiers(module).expect("deserialize");
    for id in ids {
        println!("stablecoin entry references {id}");
    }
    println!(
        "stablecoin original self {}",
        ctf::sui::original_self_address(module).expect("self")
    );
    let all =
        ctf::sui::mysten_ctf::lootboxes::stablecoin_identifiers_for_test()
            .expect("all stablecoin modules");
    for (index, ids) in all.iter().enumerate() {
        for id in ids {
            println!("stablecoin[{index}] references {id}");
        }
    }
    let ext =
        ctf::sui::mysten_ctf::lootboxes::sui_extensions_identifiers_for_test()
            .expect("all sui_extensions modules");
    for (index, ids) in ext.iter().enumerate() {
        for id in ids {
            println!("sui_extensions[{index}] references {id}");
        }
    }
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
