#![cfg(feature = "paradigm-legacy")]

//! Paradigm 2021 broker: full setup on planted WETH9 + UniswapV2
//! factory, verified unsolved.

use ctf::{Level as _, Roles, ethereum::paradigm_2021::broker};
use std::net::TcpStream;
use std::time::Duration;

fn wait_for_port(port: u16) {
    let addr = format!("127.0.0.1:{port}");
    for _ in 0..200 {
        if TcpStream::connect(&addr).is_ok() {
            return;
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    panic!("anvil did not open {port}");
}

#[tokio::test]
async fn deploys_broker_unsolved() {
    let mut anvil = std::process::Command::new("anvil")
        .args(["--port", "8913", "--code-size-limit", "50000"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("spawn anvil");
    wait_for_port(8913);

    let result = {
        let roles = Roles::new("http://127.0.0.1:8913").expect("roles");
        async {
            let target = broker::Target::set_up(&roles).await?;
            let solved = target.check(&roles).await?;
            Ok::<_, eyre::Report>((target, solved))
        }
        .await
    };

    anvil.kill().expect("kill anvil");
    anvil.wait().expect("wait anvil");

    let (target, solved) = result.expect("broker setup");
    println!("broker deployed at {}", target.address);
    assert!(!solved, "fresh setup must be unsolved");
}
