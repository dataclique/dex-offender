use ctf::Roles;
use std::io;
use std::net::TcpStream;
use std::time::Duration;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    println!("Spawning Anvil...");
    let mut anvil_cmd = std::process::Command::new("anvil");
    anvil_cmd
        .arg("--port")
        .arg("8901")
        // Paradigm's CTF launcher raises the EIP-170 code-size limit (some
        // 2023 helpers exceed 24 KiB of deployed code).
        .arg("--code-size-limit")
        .arg("50000")
        .arg("--dump-state")
        .arg("state.json")
        .arg("--state-interval")
        .arg("1");
    // Optional mainnet fork: FORK_URL (and optionally FORK_BLOCK) enable
    // the fork-gated levels (PuppetV3, CurvyPuppet). The URL is never
    // logged — it may carry credentials in its query string.
    match (std::env::var("FORK_URL"), std::env::var("FORK_BLOCK")) {
        (Ok(url), _) if !url.is_empty() => {
            println!("Forking mainnet (FORK_URL set)");
            anvil_cmd.arg("--fork-url").arg(&url);
            // Harness artifacts use a modern solc (PUSH0+); run the fork
            // with a modern spec over the historical state.
            anvil_cmd.arg("--hardfork").arg("cancun");
            if let Ok(block) = std::env::var("FORK_BLOCK")
                && !block.is_empty()
            {
                println!("Fork block {block}");
                anvil_cmd.arg("--fork-block-number").arg(&block);
            }
        }
        _ => println!("No FORK_URL set; fork-gated levels stay undeployed"),
    }
    let mut anvil = anvil_cmd.spawn()?;

    println!("Waiting for Anvil to accept connections...");
    wait_for_port("127.0.0.1", 8901, 100)?;

    println!("Connecting to Anvil...");
    let roles = Roles::new("http://127.0.0.1:8901")?;

    let result = async {
        println!("Setting up Ethereum CTFs...");
        let ethereum = ctf::ethereum::set_up_ethereum(&roles).await?;

        println!("Saving blockchain state to a file...");
        let delay = tokio::time::sleep(Duration::from_secs(2));

        println!("Saving levels to a file...");
        let ctfs = ctf::CTFs { ethereum };

        std::fs::write("ctfs.json", serde_json::to_string_pretty(&ctfs)?)?;

        delay.await;

        Ok(())
    }
    .await;

    let _ = std::process::Command::new("kill")
        .arg("-TERM")
        .arg(anvil.id().to_string())
        .status();
    let _ = anvil.wait();

    result
}

/// Poll a TCP port until it accepts a connection.
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
