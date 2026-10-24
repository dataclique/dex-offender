//! Paradigm 2021 upgrade: fork-only — deploy + call upgrade() on a
//! mainnet fork, verify isSolved() reads false before any exploit.

use ctf::{Level as _, Roles, ethereum::paradigm_2021::upgrade};
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
async fn deploys_upgrade_on_fork() {
    let Ok(fork_url) = std::env::var("FORK_URL") else {
        eprintln!("FORK_URL not set; skipping fork deploy test");
        return;
    };
    let mut anvil = std::process::Command::new("anvil")
        .args([
            "--port",
            "8911",
            "--fork-url",
            &fork_url,
            "--code-size-limit",
            "50000",
        ])
        .arg("--hardfork")
        .arg("cancun")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("spawn anvil");
    wait_for_port(8911);

    let result = {
        let roles = Roles::new("http://127.0.0.1:8911").expect("roles");
        let target = upgrade::Target::set_up(&roles).await;
        match target {
            Ok(t) => {
                let solved = t.check(&roles).await;
                (Ok(t), solved)
            }
            Err(e) => (Err(e), Err(eyre::eyre!("setup failed"))),
        }
    };

    anvil.kill().expect("kill anvil");
    anvil.wait().expect("wait anvil");

    let (target, check) = result;
    let target = target.expect("upgrade fork deployment");
    println!("upgrade deployed at {}", target.address);
    assert!(!check.expect("check"), "fresh setup must be unsolved");
}
