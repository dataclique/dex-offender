//! Focused deployment verification for the Paradigm CTF 2021 section.

use std::time::Duration;

use ctf::{Roles, ethereum::paradigm_2021::set_up_paradigm_2021};

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
async fn deploys_paradigm_2021_levels() {
    let mut anvil = std::process::Command::new("anvil")
        .args(["--port", "8905"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("spawn anvil");
    wait_for_port(8905);

    let result = {
        let roles = Roles::new("http://127.0.0.1:8905").expect("roles");
        set_up_paradigm_2021(&roles).await
    };

    anvil.kill().expect("kill anvil");
    anvil.wait().expect("wait anvil");
    let board = result.expect("paradigm 2021 deployment");

    let expect_deployed = [
        ("hello", board.hello.address != alloy::primitives::Address::ZERO),
        ("bank", board.bank.is_some()),
        ("bouncer", board.bouncer.is_some()),
        ("lockbox", board.lockbox.is_some()),
        ("market", board.market.is_some()),
        ("secure", board.secure.is_some()),
        ("vault", board.vault.is_some()),
        ("yield_aggregator", board.yield_aggregator.is_some()),
        ("babyrev", board.babyrev.is_some()),
        ("babysandbox", board.babysandbox.is_some()),
        ("jop", board.jop.is_some()),
        ("rever", board.rever.is_some()),
    ];
    for (name, ok) in expect_deployed {
        println!("{name}: {}", if ok { "DEPLOYED" } else { "FAILED" });
    }
    for (name, ok) in expect_deployed {
        assert!(ok, "{name} did not deploy");
    }
}
