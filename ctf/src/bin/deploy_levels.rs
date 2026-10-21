use ctf::Roles;
use std::io;
use std::net::TcpStream;
use std::time::Duration;

#[tokio::main]
async fn main() -> eyre::Result<()> {
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
