//! Recovery must create and fund the lost token, then detect its drain.

use ctf::{Level, Roles, ethereum::ethernaut::lvl17_recovery};
use std::{
    net::{TcpListener, TcpStream},
    process::{Child, Command, Stdio},
    time::Duration,
};

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

    fn stop(&mut self) -> std::io::Result<()> {
        if self.child.try_wait()?.is_none() {
            self.child.kill()?;
        }
        self.child.wait()?;
        Ok(())
    }
}

#[tokio::test]
async fn fresh_recovery_holds_funded_lost_token_and_reports_unsolved()
-> eyre::Result<()> {
    let mut anvil = OwnedAnvil::spawn()?;
    let result: eyre::Result<()> = async {
        anvil.wait_ready()?;
        let roles = Roles::new(&format!("http://127.0.0.1:{}", anvil.port))?;
        let target = lvl17_recovery::Target::set_up(&roles).await?;

        use alloy::providers::Provider;
        let lost = lvl17_recovery::lost_token_address_for_test(target.address);
        let balance = roles.provider().get_balance(lost).await?;
        eyre::ensure!(
            balance == alloy::primitives::U256::from(1_000_000_000_000u64),
            "setup must fund the lost token with 0.001 ETH, got {balance}"
        );
        eyre::ensure!(
            !target.check(&roles).await?,
            "funded lost token must report unsolved"
        );

        // Simulate the factory-validated solved state (zero balance at the
        // derived address) on this owned chain, then confirm the check
        // tracks that exact address.
        roles
            .provider()
            .client()
            .request::<_, ()>(
                "anvil_setBalance",
                (lost, alloy::primitives::U256::from(0u64)),
            )
            .await?;
        eyre::ensure!(
            target.check(&roles).await?,
            "zero-balance lost token must report solved"
        );
        Ok(())
    }
    .await;
    anvil.stop()?;
    result
}
