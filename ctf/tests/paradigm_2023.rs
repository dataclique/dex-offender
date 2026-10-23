//! Focused deployment verification for the Paradigm CTF 2023 section
//! (runs its own short-lived anvil, like `deploy_levels` does).

use std::time::Duration;

use ctf::{Roles, ethereum::paradigm_2023::set_up_paradigm_2023};

fn wait_for_port(port: u16) {
    let addr = format!("127.0.0.1:{port}");
    for _ in 0..50 {
        if std::net::TcpStream::connect(&addr).is_ok() {
            return;
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    panic!("anvil did not open {port}");
}

#[tokio::test]
async fn deploys_paradigm_2023_levels() {
    let mut anvil = std::process::Command::new("anvil")
        .args(["--port", "8902", "--code-size-limit", "50000"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("spawn anvil");
    wait_for_port(8902);

    let result = {
        let roles = Roles::new("http://127.0.0.1:8902").expect("roles");
        set_up_paradigm_2023(&roles).await
    };

    anvil.kill().expect("kill anvil");
    anvil.wait().expect("wait anvil");
    let board = result.expect("paradigm 2023 deployment");

    // The two known-good anchors.
    assert!(board.hello_world.address != alloy::primitives::Address::ZERO);
    assert!(board.cosmic_radiation.address != alloy::primitives::Address::ZERO);

    // The seven newly wired set_ups must land on this local chain.
    let expect_deployed = [
        ("dragon_tyrant", board.dragon_tyrant.is_some()),
        ("dropper", board.dropper.is_some()),
        ("enterprise_blockchain", board.enterprise_blockchain.is_some()),
        ("free_real_estate", board.free_real_estate.is_some()),
        ("one_hundred_percent", board.one_hundred_percent.is_some()),
        ("suspicious_charity", board.suspicious_charity.is_some()),
        ("dai_plus_plus", board.dai_plus_plus.is_some()),
    ];
    for (name, ok) in expect_deployed {
        println!("{name}: {}", if ok { "DEPLOYED" } else { "FAILED" });
    }
    for (name, ok) in expect_deployed {
        assert!(ok, "{name} did not deploy");
    }
}
