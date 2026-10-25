//! Paradigm 2021 swap: full setup on planted stablecoin mocks (DAI,
//! TUSD, USDC, USDT), Router01-address router bytecode, factory, WETH9
//! and four seeded pairs; verified unsolved.

use ctf::{Level as _, Roles, ethereum::paradigm_2021::swap};
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
async fn deploys_swap_unsolved() {
    let mut anvil = std::process::Command::new("anvil")
        .args(["--port", "8916", "--code-size-limit", "50000"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("spawn anvil");
    wait_for_port(8916);

    let result = {
        let roles = Roles::new("http://127.0.0.1:8916").expect("roles");
        async {
            let target = swap::Target::set_up(&roles).await?;
            let solved = target.check(&roles).await?;
            Ok::<_, eyre::Report>((target, solved))
        }
        .await
    };

    anvil.kill().expect("kill anvil");
    anvil.wait().expect("wait anvil");

    let (target, solved) = result.expect("swap setup");
    println!("swap deployed at {}", target.address);
    assert!(!solved, "fresh setup must be unsolved");
}
