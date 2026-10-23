//! Focused deployment verification for the Paradigm CTF 2022 section.

use std::time::Duration;

use ctf::{Roles, ethereum::paradigm_2022::set_up_paradigm_2022};

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
async fn deploys_paradigm_2022_levels() {
    let mut anvil = std::process::Command::new("anvil")
        .args(["--port", "8910", "--code-size-limit", "50000"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("spawn anvil");
    wait_for_port(8910);

    let result = {
        let roles = Roles::new("http://127.0.0.1:8910").expect("roles");
        set_up_paradigm_2022(&roles).await
    };

    anvil.kill().expect("kill anvil");
    anvil.wait().expect("wait anvil");
    let board = result.expect("paradigm 2022 deployment");

    let expect_deployed = [
        ("fun_reversing_challenge", board.fun_reversing_challenge.is_some()),
        ("just_in_time", board.just_in_time.is_some()),
        ("lockbox2", board.lockbox2.is_some()),
        ("merkledrop", board.merkledrop.is_some()),
        ("random", board.random.is_some()),
        ("sourcecode", board.sourcecode.is_some()),
        ("vanity", board.vanity.is_some()),
    ];
    for (name, ok) in expect_deployed {
        println!("{name}: {}", if ok { "DEPLOYED" } else { "FAILED" });
    }
    for (name, ok) in expect_deployed {
        assert!(ok, "{name} did not deploy");
    }
}
