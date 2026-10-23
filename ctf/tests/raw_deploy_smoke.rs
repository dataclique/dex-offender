//! Minimal raw-create smoke test for the alloy transaction path used by
//! the dragon-tyrant Factory deployment.

use alloy::{
    network::{ReceiptResponse, TransactionBuilder},
    primitives::Bytes,
    providers::Provider,
    rpc::types::TransactionRequest,
};
use ctf::Roles;

fn wait_for_port(port: u16) {
    let addr = format!("127.0.0.1:{port}");
    for _ in 0..50 {
        if std::net::TcpStream::connect(&addr).is_ok() {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
    panic!("anvil did not open {port}");
}

#[tokio::test]
async fn raw_deploy_smoke() {
    let mut anvil = std::process::Command::new("anvil")
        .args(["--port", "8903"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("spawn anvil");
    wait_for_port(8903);

    let result = {
        let roles = Roles::new("http://127.0.0.1:8903").expect("roles");
        let code: Bytes =
            "0x600a600c600039600a6000f360fe6000815f55".parse().unwrap();
        eprintln!("[smoke] sending raw create");
        let receipt = roles
            .provider()
            .send_transaction(
                TransactionRequest::default()
                    .create()
                    .with_from(roles.deployer.address())
                    .with_input(code),
            )
            .await
            .expect("send")
            .get_receipt()
            .await
            .expect("receipt");
        eprintln!("[smoke] receipt: {:?}", receipt.contract_address());
        receipt
    };

    anvil.kill().expect("kill anvil");
    anvil.wait().expect("wait anvil");
    assert!(result.contract_address().is_some());
}
