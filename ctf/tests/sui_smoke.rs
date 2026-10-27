#![cfg(feature = "sui")]

//! Sui harness-wiring smoke: publish the sui_smoke Move fixture from a
//! genesis-funded role through the pure-SDK path (builder + in-harness
//! signing + gRPC execution), read the package object back, persist the
//! board through the ctfs.json state shape, and tear the localnet down.
//! Skips without SUI_LOCALNET_TEST.

use ctf::sui::{SmokeTarget, SuiBoard, SuiLocalnet};
use sui_sdk_types::Address;

#[tokio::test]
async fn publish_object_read_and_state_round_trip() {
    if std::env::var("SUI_LOCALNET_TEST").is_err() {
        eprintln!("SUI_LOCALNET_TEST not set; skipping smoke test");
        return;
    }

    let (localnet, roles) =
        SuiLocalnet::start_with_roles().await.expect("localnet with roles");

    // the fixture's compiled bytecode (sui move build output)
    let module_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/fixtures/sui_smoke/build/sui_smoke/bytecode_modules/smoke.mv"
    );
    let modules = vec![
        std::fs::read(module_path)
            .expect("fixture bytecode (run sui move build)"),
    ];

    let outcome = localnet
        .publish(
            &roles.deployer,
            modules,
            vec![
                Address::from_static(
                    "0x0000000000000000000000000000000000000000000000000000000000000001",
                ), // MoveStdlib
                Address::from_static(
                    "0x0000000000000000000000000000000000000000000000000000000000000002",
                ), // Sui
            ],
        )
        .await
        .expect("publish smoke package");
    let package = sui_sdk_types::Address::from_hex(&outcome.package)
        .expect("package id parses");
    println!("smoke package published at {package}");

    // object read: the immutable package object must be retrievable;
    // indexing lags execution by a checkpoint tick, so retry bounded
    let mut client = localnet.client();
    let deadline =
        std::time::Instant::now() + std::time::Duration::from_secs(30);
    let object = loop {
        let mut ledger = client.ledger_client();
        let mut request =
            sui_rpc::proto::sui::rpc::v2::GetObjectRequest::const_default();
        request.object_id = Some(package.to_string());
        match ledger.get_object(request).await {
            Ok(response) => {
                break response.into_inner().object.expect("object body");
            }
            Err(_) if std::time::Instant::now() < deadline => {
                tokio::time::sleep(std::time::Duration::from_millis(1000))
                    .await;
            }
            Err(e) => panic!("package object read never succeeded: {e:?}"),
        }
    };
    let read_id = object.object_id.expect("object id field");
    assert_eq!(
        read_id.replace("0x", "").to_lowercase(),
        package.to_string().replace("0x", "").to_lowercase(),
        "read-back object id must match the published package"
    );

    // state wiring: the board serializes into the exact ctfs.json shape
    let board =
        SuiBoard { smoke: Some(SmokeTarget { package }), ..Default::default() };
    let json = serde_json::to_value(&board).expect("board serialize");
    assert_eq!(
        json["smoke"]["package"].as_str().expect("package hex"),
        package.to_string(),
        "state shape"
    );
    let parsed: SuiBoard =
        serde_json::from_value(json).expect("board deserialize");
    assert_eq!(parsed, board, "state round trip");

    // lifecycle cleanup: port freed, scratch removed
    let scratch = localnet.scratch_dir().to_path_buf();
    localnet.teardown().await.expect("teardown");
    assert!(!scratch.exists(), "scratch removed");
    assert!(
        std::net::TcpStream::connect(("127.0.0.1", 9000)).is_err(),
        "rpc port freed"
    );
}

#[test]
fn missing_sui_section_parses_as_default_board() {
    // the REAL legacy ctfs.json (no sui section) must keep parsing with
    // the new sui field defaulting
    let raw = std::fs::read_to_string("../ctfs.json").expect("repo ctfs.json");
    let ctfs: ctf::CTFs =
        serde_json::from_str(&raw).expect("legacy ctfs.json parses");
    assert_eq!(ctfs.sui, SuiBoard::default(), "missing sui section defaults");
}
