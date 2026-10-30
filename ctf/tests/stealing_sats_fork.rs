#![cfg(feature = "paradigm-legacy")]

//! Paradigm 2022 stealing-sats: fork deploy at the verified 2022-era
//! block (TARGET holds ~0.29 WBTC at 15707136) + unsolved check, and
//! the no-fork guard failing fast.

use ctf::{Level as _, Roles, ethereum::paradigm_2022::stealing_sats};
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

async fn run_on(
    port: u16,
    fork_url: Option<&str>,
) -> (eyre::Result<bool>, Option<stealing_sats::Target>) {
    let mut args = vec![
        "--port".to_string(),
        port.to_string(),
        "--code-size-limit".to_string(),
        "50000".to_string(),
    ];
    if let Some(url) = fork_url {
        args.push("--fork-url".to_string());
        args.push(url.to_string());
        args.push("--hardfork".to_string());
        args.push("cancun".to_string());
        if let Ok(block) = std::env::var("FORK_BLOCK") {
            args.push("--fork-block-number".to_string());
            args.push(block);
        }
    }
    let mut anvil = std::process::Command::new("anvil")
        .args(&args)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("spawn anvil");
    wait_for_port(port);

    let result = {
        let roles =
            Roles::new(&format!("http://127.0.0.1:{port}")).expect("roles");
        async {
            let target = stealing_sats::Target::set_up(&roles).await?;
            let solved = target.check(&roles).await?;
            Ok::<_, eyre::Report>((solved, target))
        }
        .await
    };

    anvil.kill().expect("kill anvil");
    anvil.wait().expect("wait anvil");
    match result {
        Ok((solved, target)) => (Ok(solved), Some(target)),
        Err(e) => (Err(e), None),
    }
}

#[tokio::test]
async fn deploys_stealing_sats_unsolved_on_fork() {
    let Ok(fork_url) = std::env::var("FORK_URL") else {
        eprintln!("FORK_URL not set; skipping fork deploy test");
        return;
    };
    let (solved, target) = run_on(8924, Some(&fork_url)).await;
    let target = target.expect("stealing-sats fork deployment");
    println!("stealing-sats deployed at {}", target.address);
    assert!(!solved.expect("check"), "fresh setup must be unsolved");
}

#[tokio::test]
async fn fails_fast_without_fork() {
    let (result, target) = run_on(8925, None).await;
    assert!(target.is_none(), "no local deployment without a fork");
    let err = result.expect_err("set_up must fail on a plain chain");
    assert!(
        err.to_string().contains("FORK_URL"),
        "error must name the fork requirement, got: {err}"
    );
}
