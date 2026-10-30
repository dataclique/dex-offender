//! Deployed Ethernaut levels must start checkable and unsolved (owned Anvil).

use ctf::{Level, Roles, ethereum::ethernaut};
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

    fn stop(&mut self) -> std::io::Result<()> {
        if self.child.try_wait()?.is_none() {
            self.child.kill()?;
        }
        self.child.wait()?;
        Ok(())
    }
}

macro_rules! fresh_check {
    ($test_name:ident, $module:path) => {
        #[tokio::test]
        async fn $test_name() -> eyre::Result<()> {
            let mut anvil = OwnedAnvil::spawn()?;
            let url = format!("http://127.0.0.1:{}", anvil.port);
            let result: eyre::Result<()> = async {
                anvil.wait_ready()?;
                let roles = Roles::new(&url)?;
                let level = <$module as Level>::set_up(&roles)
                    .await
                    .map_err(|e| eyre::eyre!("set_up failed: {e:#}"))?;
                let checked = level.check(&roles).await;
                println!("fresh check = {:?}", checked);
                // The check must first of all not error on a fresh
                // deployment, and it must report unsolved.
                let solved = checked
                    .map_err(|e| eyre::eyre!("fresh check errored: {e:#}"))?;
                eyre::ensure!(!solved, "fresh deployment reports solved");
                Ok(())
            }
            .await;
            anvil.stop()?;
            result
        }
    };
}

fresh_check!(dex_fresh, ethernaut::lvl22_dex::Target);
fresh_check!(dex_two_fresh, ethernaut::lvl23_dex_two::Target);
fresh_check!(good_samaritan_fresh, ethernaut::lvl27_good_samaritan::Target);
fresh_check!(cashback_fresh, ethernaut::lvl36_cashback::Target);

// Delegation deployed the same contract twice; its fresh regression pins
// that the level's two contracts are actually distinct.
#[tokio::test]
async fn delegation_deploys_distinct_contracts() -> eyre::Result<()> {
    let mut anvil = OwnedAnvil::spawn()?;
    let url = format!("http://127.0.0.1:{}", anvil.port);
    let result: eyre::Result<()> = async {
        anvil.wait_ready()?;
        let roles = Roles::new(&url)?;
        let level = ethernaut::lvl06_delegate::Target::set_up(&roles).await?;
        use alloy::providers::Provider;
        let code1 =
            roles.provider().get_code_at(level.delegation_address).await?;
        let code2 =
            roles.provider().get_code_at(level.delegate_address).await?;
        eyre::ensure!(
            !code1.is_empty() && !code2.is_empty(),
            "delegation setup left a contract undeployed"
        );
        eyre::ensure!(
            level.delegation_address != level.delegate_address,
            "delegation deployed the same contract twice"
        );
        eyre::ensure!(
            !level.check(&roles).await?,
            "fresh delegation reports solved"
        );
        Ok(())
    }
    .await;
    anvil.stop()?;
    result
}
