use ctf::Roles;
use std::io;
use std::net::TcpStream;
use std::time::Duration;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // default port is 8545 but we don't want to clash if Anvil
    // is already running
    println!("Spawning Anvil...");
    let mut anvil = std::process::Command::new("anvil")
        .arg("--port")
        .arg("8901")
        .arg("--dump-state")
        .arg("state.json")
        .arg("--state-interval")
        .arg("1")
        .spawn()?;

    println!("Waiting for Anvil to accept connections...");
    wait_for_port("127.0.0.1", 8901, 100)?;

    println!("Connecting to Anvil...");
    let roles = Roles::new("http://127.0.0.1:8901")?;

    let result = async {
        println!("Setting up Ethernaut...");
        let ethernaut = ctf::ethernaut::set_up_ethernaut(&roles).await?;

        println!("Setting up Paradigm 2023...");
        let paradigm_2023 =
            ctf::paradigm_2023::set_up_paradigm_2023(&roles).await?;

        println!("Setting up Damn Vulnerable DeFi...");
        let damn_vulnerable_defi =
            ctf::damn_vulnerable_defi::set_up_damn_vulnerable_defi(&roles)
                .await?;

        // we need to wait for at least a second for Anvil to save
        // the state
        println!("Saving blockchain state to a file...");
        let delay = tokio::time::sleep(Duration::from_secs(2));

        println!("Saving levels to a file...");
        let ctfs = ctf::CTFs { ethernaut, paradigm_2023, damn_vulnerable_defi };

        std::fs::write("ctfs.json", serde_json::to_string_pretty(&ctfs)?)?;

        delay.await;

        Ok(())
    }
    .await;

    // Send SIGTERM so anvil dumps state on shutdown (--dump-state)
    let _ = std::process::Command::new("kill")
        .arg("-TERM")
        .arg(anvil.id().to_string())
        .status();
    let _ = anvil.wait();

    result
}

/// Poll a TCP port until it accepts a connection, up to `attempts` times
/// with 100ms between tries. Anvil needs a moment to start listening.
fn wait_for_port(host: &str, port: u16, attempts: u32) -> io::Result<()> {
    let addr = format!("{host}:{port}");
    for i in 0..attempts {
        if TcpStream::connect(&addr).is_ok() {
            println!("Anvil is ready after {} attempt(s).", i + 1);
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    Err(io::Error::other(format!(
        "anvil at {addr} did not accept connections in time"
    )))
}
