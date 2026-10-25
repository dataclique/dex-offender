//! Sui Rust SDK boundary test: connect to a running localnet via the
//! official MystenLabs/sui-rust-sdk client and perform a real read
//! (latest checkpoint through the gRPC LedgerService). Skips when no
//! localnet is up.

use sui_rpc::proto::sui::rpc::v2::GetCheckpointRequest;
use sui_rpc::proto::sui::rpc::v2::get_checkpoint_request::CheckpointId;

#[tokio::test]
async fn reads_latest_checkpoint_from_localnet() {
    if std::env::var("SUI_LOCALNET_UP").is_err() {
        eprintln!("SUI_LOCALNET_UP not set; skipping localnet boundary test");
        return;
    }

    let mut client: sui_rpc::Client =
        sui_rpc::Client::new("http://127.0.0.1:9000")
            .expect("grpc client construction");

    // no checkpoint id = latest; default mask returns sequence + digest
    let mut ledger = client.ledger_client();
    let response = ledger
        .get_checkpoint(GetCheckpointRequest::const_default())
        .await
        .expect("grpc get_checkpoint call");
    let checkpoint = response.into_inner().checkpoint.expect("checkpoint body");
    let seq = checkpoint.sequence_number.expect("checkpoint sequence");
    println!("localnet latest checkpoint sequence: {seq}");
    // fresh localnet ticks checkpoints continuously; assert a real,
    // recently-genesis read rather than an exact sequence
    assert!(
        seq < 1000,
        "checkpoint read must come from fresh localnet, got {seq}"
    );
    let _ = CheckpointId::SequenceNumber(0); // api surface compile check
}
