//! Deployed DVD levels must start unsolved (owned Anvil per test).

use ctf::{Level, Roles, ethereum::damn_vulnerable_defi};
use std::{
    net::{TcpListener, TcpStream},
    process::{Child, Command, Stdio},
    time::Duration,
};

/// Owned Anvil on an OS-assigned port; killed and reaped on drop.
struct OwnedAnvil {
    child: Child,
    port: u16,
}

impl OwnedAnvil {
    fn spawn() -> std::io::Result<Self> {
        let reservation = TcpListener::bind(("127.0.0.1", 0))?;
        let port = reservation.local_addr()?.port();
        drop(reservation);
        let child = Command::new("anvil")
            .args(["--port", &port.to_string(), "--quiet"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        Ok(Self { child, port })
    }

    fn wait_ready(&mut self) -> std::io::Result<()> {
        for _ in 0..100 {
            if let Some(status) = self.child.try_wait()? {
                return Err(std::io::Error::other(format!(
                    "owned Anvil exited: {status}"
                )));
            }
            if TcpStream::connect(("127.0.0.1", self.port)).is_ok() {
                return Ok(());
            }
            std::thread::sleep(Duration::from_millis(50));
        }
        Err(std::io::Error::other("owned Anvil never became ready"))
    }

    fn url(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }

    fn stop(&mut self) -> std::io::Result<()> {
        if self.child.try_wait()?.is_none() {
            self.child.kill()?;
        }
        self.child.wait()?;
        Ok(())
    }
}

#[tokio::test]
async fn free_rider_starts_unsolved() -> eyre::Result<()> {
    let mut anvil = OwnedAnvil::spawn()?;
    let result: eyre::Result<()> = async {
        anvil.wait_ready()?;
        let roles = Roles::new(&anvil.url())?;
        let level = damn_vulnerable_defi::free_rider::Target::set_up(&roles)
            .await
            .map_err(|error| {
                eyre::eyre!("free_rider setup failed: {error:#}")
            })?;
        eyre::ensure!(
            !level.check(&roles).await?,
            "free_rider: fresh deployment reports solved"
        );
        Ok(())
    }
    .await;
    anvil.stop()?;
    result
}

#[tokio::test]
async fn puppet_starts_unsolved() -> eyre::Result<()> {
    let mut anvil = OwnedAnvil::spawn()?;
    let result: eyre::Result<()> = async {
        anvil.wait_ready()?;
        let roles = Roles::new(&anvil.url())?;
        let level =
            damn_vulnerable_defi::puppet::Target::set_up(&roles).await?;
        eyre::ensure!(
            !level.check(&roles).await?,
            "puppet: fresh deployment reports solved"
        );
        Ok(())
    }
    .await;
    anvil.stop()?;
    result
}
