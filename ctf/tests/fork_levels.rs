//! Focused fork deployment verification for the fork-gated DVD levels.
//!
//! Requires FORK_URL (and FORK_BLOCK) in the environment; skips silently
//! when unset so the default local test suite stays green.

use std::time::Duration;

use ctf::{
    Level as _, Roles,
    ethereum::damn_vulnerable_defi::{curvy_puppet, puppet_v3},
};

fn wait_for_port(port: u16) {
    let addr = format!("127.0.0.1:{port}");
    for _ in 0..200 {
        if std::net::TcpStream::connect(&addr).is_ok() {
            return;
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    panic!("anvil did not open {port}");
}

#[tokio::test]
async fn deploys_puppet_v3_on_fork() {
    let Ok(fork_url) = std::env::var("FORK_URL") else {
        eprintln!("FORK_URL not set; skipping fork deploy test");
        return;
    };
    let mut anvil = std::process::Command::new("anvil");
    anvil
        .args(["--port", "8906", "--fork-url", &fork_url])
        // Our harness artifacts are compiled with a modern solc (PUSH0+);
        // run the fork with a modern spec over the historical state.
        .arg("--hardfork")
        .arg("cancun")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    if let Ok(block) = std::env::var("FORK_BLOCK")
        && !block.is_empty()
    {
        anvil.args(["--fork-block-number", &block]);
    }
    let mut anvil = anvil.spawn().expect("spawn anvil");
    wait_for_port(8906);

    let result = {
        let roles = Roles::new("http://127.0.0.1:8906").expect("roles");
        puppet_v3::Target::set_up(&roles).await
    };

    anvil.kill().expect("kill anvil");
    anvil.wait().expect("wait anvil");

    let target = result.expect("puppet v3 fork deployment");
    println!("puppet_v3 deployed at {}", target.address);
    assert_ne!(
        target.address,
        alloy::primitives::Address::ZERO,
        "puppet v3 address must be real"
    );
}

#[tokio::test]
async fn deploys_curvy_puppet_on_fork() {
    let Ok(fork_url) = std::env::var("FORK_URL") else {
        eprintln!("FORK_URL not set; skipping fork deploy test");
        return;
    };
    let mut anvil = std::process::Command::new("anvil");
    anvil
        .args([
            "--port",
            "8908",
            "--fork-url",
            &fork_url,
            "--fork-block-number",
            "20190356",
        ])
        .arg("--hardfork")
        .arg("cancun")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    let mut anvil = anvil.spawn().expect("spawn anvil");
    wait_for_port(8908);

    let result = {
        let roles = Roles::new("http://127.0.0.1:8908").expect("roles");
        curvy_puppet::Target::set_up(&roles).await
    };

    anvil.kill().expect("kill anvil");
    anvil.wait().expect("wait anvil");

    let target = result.expect("curvy puppet fork deployment");
    println!("curvy_puppet deployed at {}", target.address);
    assert_ne!(
        target.address,
        alloy::primitives::Address::ZERO,
        "curvy puppet address must be real"
    );
}
