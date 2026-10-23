//! Focused deployment verification for WalletMining (local chain).

use std::time::Duration;

use ctf::{Level as _, Roles, ethereum::damn_vulnerable_defi::wallet_mining};

fn wait_for_port(port: u16) {
    let addr = format!("127.0.0.1:{port}");
    for _ in 0..100 {
        if std::net::TcpStream::connect(&addr).is_ok() {
            return;
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    panic!("anvil did not open {port}");
}

#[tokio::test]
async fn deploys_wallet_mining() {
    let mut anvil = std::process::Command::new("anvil")
        .args(["--port", "8909", "--code-size-limit", "50000"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("spawn anvil");
    wait_for_port(8909);

    let result = {
        let roles = Roles::new("http://127.0.0.1:8909").expect("roles");
        wallet_mining::Target::set_up(&roles).await
    };

    anvil.kill().expect("kill anvil");
    anvil.wait().expect("wait anvil");

    let target = result.expect("wallet mining deployment");
    println!("wallet_mining deployed at {}", target.address);
    assert_ne!(
        target.address,
        alloy::primitives::Address::ZERO,
        "wallet mining address must be real"
    );
}
