//! Sui chain-core lifecycle: guarded localnet start with genesis-funded
//! in-harness roles, and teardown that frees the RPC port and removes
//! scratch. Skips when SUI_LOCALNET_TEST isn't set (needs the
//! dev-shell sui or SUI_BIN).

use ctf::sui::SuiLocalnet;

#[tokio::test]
async fn localnet_lifecycle_and_funded_roles() {
    if std::env::var("SUI_LOCALNET_TEST").is_err() {
        eprintln!("SUI_LOCALNET_TEST not set; skipping localnet core test");
        return;
    }

    // start + readiness + role funding (derivation end-to-end check)
    let (localnet, roles) = SuiLocalnet::start_with_roles()
        .await
        .expect("localnet start with roles");

    let mut client = localnet.client();
    let mut ledger = client.ledger_client();
    let cp = ledger
        .get_checkpoint(
            sui_rpc::proto::sui::rpc::v2::GetCheckpointRequest::const_default(),
        )
        .await
        .expect("checkpoint read")
        .into_inner()
        .checkpoint
        .expect("checkpoint body")
        .sequence_number
        .expect("sequence");
    println!("localnet checkpoint after start: {cp}");

    for (name, actor) in [
        ("deployer", &roles.deployer),
        ("some_user", &roles.some_user),
        ("offender", &roles.offender),
    ] {
        let balance = actor.gas_balance().await.expect("balance read");
        println!("{name} gas balance: {balance}");
        assert!(balance > 0, "{name} must be genesis-funded");
    }

    // teardown: port freed, scratch removed
    let scratch = localnet.scratch_dir().to_path_buf();
    localnet.teardown().await.expect("teardown");
    assert!(!scratch.exists(), "scratch must be removed");
    assert!(
        std::net::TcpStream::connect(("127.0.0.1", 9000)).is_err(),
        "rpc port must be free after teardown"
    );

    // silence unused-roles warning when assertions compile out
    let _ = roles;
}
