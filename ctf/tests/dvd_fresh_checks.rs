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
async fn withdrawal_starts_unsolved() -> eyre::Result<()> {
    let mut anvil = OwnedAnvil::spawn()?;
    let result: eyre::Result<()> = async {
        anvil.wait_ready()?;
        let roles = Roles::new(&anvil.url())?;
        let level = damn_vulnerable_defi::withdrawal::Target::set_up(&roles)
            .await
            .map_err(|error| {
                eyre::eyre!("withdrawal setup failed: {error:#}")
            })?;
        eyre::ensure!(
            !level.check(&roles).await?,
            "withdrawal: fresh deployment reports solved"
        );
        Ok(())
    }
    .await;
    anvil.stop()?;
    result
}

#[tokio::test]
async fn shards_starts_unsolved_with_upstream_state() -> eyre::Result<()> {
    let mut anvil = OwnedAnvil::spawn()?;
    let result: eyre::Result<()> = async {
        anvil.wait_ready()?;
        let roles = Roles::new(&anvil.url())?;
        let level =
            damn_vulnerable_defi::shards::Target::set_up(&roles).await?;
        let marketplace =
            damn_vulnerable_defi::shards::ShardsNFTMarketplace::new(
                level.address,
                roles.provider(),
            );
        let token = damn_vulnerable_defi::shards::DamnValuableToken::new(
            level.token,
            roles.provider(),
        );
        // Upstream initial state: one open offer, fee escrow in the
        // marketplace, staking rewards intact, player holds nothing.
        eyre::ensure!(
            marketplace.offerCount().call().await? == 1,
            "shards: seller offer missing"
        );
        eyre::ensure!(
            marketplace.feesInBalance().call().await?
                > alloy::primitives::U256::ZERO,
            "shards: no fee escrow"
        );
        eyre::ensure!(
            token.balanceOf(level.staking).call().await?
                == alloy::primitives::U256::from(
                    100_000_000_000_000_000_000_000u128
                ),
            "shards: staking rewards wrong"
        );
        eyre::ensure!(
            !level.check(&roles).await?,
            "shards: fresh deployment reports solved"
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

#[tokio::test]
async fn the_rewarder_starts_unsolved_with_upstream_state() -> eyre::Result<()>
{
    let mut anvil = OwnedAnvil::spawn()?;
    let result: eyre::Result<()> = async {
        anvil.wait_ready()?;
        let roles = Roles::new(&anvil.url())?;
        let level = damn_vulnerable_defi::the_rewarder::Target::set_up(&roles)
            .await
            .map_err(|error| {
                eyre::eyre!("the_rewarder setup failed: {error:#}")
            })?;
        let dvt = damn_vulnerable_defi::the_rewarder::DamnValuableToken::new(
            level.dvt,
            roles.provider(),
        );
        let weth = damn_vulnerable_defi::the_rewarder::WETH::new(
            level.weth,
            roles.provider(),
        );
        let distributor =
            damn_vulnerable_defi::the_rewarder::TheRewarderDistributor::new(
                level.address,
                roles.provider(),
            );
        // Upstream initial state: both batches exist, ~all funds still in
        // the distributor (only Alice claimed), and the level is unsolved.
        eyre::ensure!(
            dvt.balanceOf(level.address).call().await?
                > alloy::primitives::U256::from(9_000_000_000_000_000_000u64),
            "the_rewarder: distributor DVT balance not upstream-like"
        );
        eyre::ensure!(
            weth.balanceOf(level.address).call().await?
                > alloy::primitives::U256::from(900_000_000_000_000_000u64),
            "the_rewarder: distributor WETH balance not upstream-like"
        );
        eyre::ensure!(
            distributor.getNextBatchNumber(level.dvt).call().await?
                == alloy::primitives::U256::from(1u64),
            "the_rewarder: DVT batch missing"
        );
        eyre::ensure!(
            !level.check(&roles).await?,
            "the_rewarder: fresh deployment reports solved"
        );
        Ok(())
    }
    .await;
    anvil.stop()?;
    result
}
