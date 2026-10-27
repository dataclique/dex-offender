use ctf::Roles;
use eyre::{WrapErr, eyre};
use std::{
    ffi::OsStr,
    net::{SocketAddr, TcpListener},
    process::{ExitStatus, Stdio},
    time::Duration,
};
use tokio::{
    io::{AsyncBufRead, AsyncBufReadExt, BufReader, Lines},
    process::{Child, Command},
    time::timeout,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Non-secret override for isolated deployment runs; practice uses 8545.
    let port = match std::env::var("DEPLOY_PORT") {
        Ok(value) => value
            .parse::<u16>()
            .wrap_err("DEPLOY_PORT must be a port number")?,
        Err(std::env::VarError::NotPresent) => 8901,
        Err(error) => return Err(error.into()),
    };
    eyre::ensure!(port != 0, "DEPLOY_PORT must be between 1 and 65535");
    let endpoint = SocketAddr::from(([127, 0, 0, 1], port));
    let reservation = TcpListener::bind(endpoint).wrap_err_with(|| {
        format!(
            "deployment port {port} is already in use; not touching that node"
        )
    })?;

    println!("Spawning owned Anvil on port {port}...");
    let mut command = Command::new("anvil");
    command
        .kill_on_drop(true)
        .env("NO_COLOR", "1")
        .stdout(Stdio::piped())
        .arg("--host")
        .arg("127.0.0.1")
        .arg("--port")
        .arg(port.to_string())
        .arg("--code-size-limit")
        .arg("50000")
        .arg("--dump-state")
        .arg("state.json")
        .arg("--state-interval")
        .arg("1");
    match (std::env::var("FORK_URL"), std::env::var("FORK_BLOCK")) {
        (Ok(url), _) if !url.is_empty() => {
            println!("Forking mainnet (FORK_URL set)");
            command.arg("--fork-url").arg(url).arg("--hardfork").arg("cancun");
            if let Ok(block) = std::env::var("FORK_BLOCK")
                && !block.is_empty()
            {
                command.arg("--fork-block-number").arg(block);
            }
        }
        _ => println!("No FORK_URL set; fork-gated levels stay undeployed"),
    }
    drop(reservation);
    let mut anvil = command.spawn()?;
    let mut output_task = None;

    let result: eyre::Result<()> = async {
        let stdout = anvil
            .stdout
            .take()
            .ok_or_else(|| eyre!("owned Anvil stdout is missing"))?;
        let mut lines = BufReader::new(stdout).lines();
        println!(
            "Waiting for the owned Anvil to report its listening endpoint..."
        );
        timeout(
            Duration::from_secs(10),
            confirm_owned_listener(&mut lines, endpoint),
        )
        .await
        .wrap_err("owned Anvil startup timed out")??;
        // Drain private startup/account data and RPC chatter without displaying it.
        output_task = Some(tokio::spawn(async move {
            while lines.next_line().await?.is_some() {}
            Ok::<_, std::io::Error>(())
        }));
        eyre::ensure!(
            anvil.try_wait()?.is_none(),
            "owned Anvil exited after readiness"
        );
        let roles = Roles::new(&format!("http://{endpoint}"))?;

        println!("Setting up Ethereum CTFs...");
        let ethereum = timeout(
            Duration::from_secs(300),
            ctf::ethereum::set_up_ethereum(&roles),
        )
        .await
        .wrap_err("deployment did not finish within 300 seconds")??;
        let ctfs = ctf::CTFs {
            ethereum,
            #[cfg(feature = "sui")]
            sui: ctf::sui::SuiBoard::default(),
        };
        println!("Saving levels to ctfs.json...");
        std::fs::write("ctfs.json", serde_json::to_string_pretty(&ctfs)?)?;
        tokio::time::sleep(Duration::from_secs(2)).await;
        Ok(())
    }
    .await;

    let shutdown = stop_owned_anvil(&mut anvil, OsStr::new("kill")).await;
    let drained: eyre::Result<()> = if let Some(mut task) = output_task {
        match timeout(Duration::from_secs(1), &mut task).await {
            Ok(Ok(read)) => read.map_err(Into::into),
            Ok(Err(error)) => Err(error.into()),
            Err(error) => {
                task.abort();
                Err(error.into())
            }
        }
    } else {
        Ok(())
    };
    deployment_outcome(result, shutdown, drained)?;
    println!(
        "Deployment complete: ctfs.json and state.json saved; owned Anvil stopped."
    );
    Ok(())
}

async fn confirm_owned_listener<R: AsyncBufRead + Unpin>(
    lines: &mut Lines<R>,
    expected: SocketAddr,
) -> eyre::Result<()> {
    while let Some(line) = lines.next_line().await? {
        if let Some(address) = line.strip_prefix("Listening on ") {
            let address: SocketAddr = address.trim().parse().wrap_err(
                "owned Anvil reported a malformed listening endpoint",
            )?;
            eyre::ensure!(
                address == expected,
                "owned Anvil reported another endpoint; refusing RPC"
            );
            return Ok(());
        }
    }
    Err(eyre!("owned Anvil exited without confirming its listening endpoint"))
}

async fn force_stop(child: &mut Child) -> eyre::Result<()> {
    timeout(Duration::from_secs(3), child.kill()).await.wrap_err(
        "owned Anvil could not be reaped after forced termination",
    )??;
    Ok(())
}

async fn stop_owned_anvil(
    child: &mut Child,
    signal_program: &OsStr,
) -> eyre::Result<()> {
    if let Some(status) = child.try_wait()? {
        return Err(eyre!("owned Anvil exited unexpectedly: {status}"));
    }
    let pid = child
        .id()
        .ok_or_else(|| eyre!("owned Anvil has no running process"))?;
    let signal = timeout(
        Duration::from_secs(2),
        Command::new(signal_program)
            .kill_on_drop(true)
            .arg("-TERM")
            .arg(pid.to_string())
            .status(),
    )
    .await;
    if !matches!(signal, Ok(Ok(status)) if status.success()) {
        force_stop(child).await?;
        return Err(eyre!(
            "graceful Anvil shutdown failed; owned process force-stopped"
        ));
    }
    let status = match timeout(Duration::from_secs(5), child.wait()).await {
        Ok(Ok(status)) => status,
        _ => {
            force_stop(child).await?;
            return Err(eyre!(
                "owned Anvil did not stop gracefully; force-stopped"
            ));
        }
    };
    eyre::ensure!(
        expected_shutdown(status),
        "owned Anvil shutdown failed: {status}"
    );
    Ok(())
}

fn deployment_outcome(
    result: eyre::Result<()>,
    shutdown: eyre::Result<()>,
    drained: eyre::Result<()>,
) -> eyre::Result<()> {
    let mut outcome = result;
    for (label, next) in
        [("Anvil shutdown", shutdown), ("Anvil output drain", drained)]
    {
        if let Err(additional) = next {
            outcome = match outcome {
                Ok(()) => Err(additional.wrap_err(label)),
                Err(primary) => Err(primary
                    .wrap_err(format!("{label} also failed: {additional:#}"))),
            };
        }
    }
    outcome
}

fn expected_shutdown(status: ExitStatus) -> bool {
    status.success() || {
        #[cfg(unix)]
        {
            use std::os::unix::process::ExitStatusExt;
            status.signal() == Some(15)
        }
        #[cfg(not(unix))]
        {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simultaneous_failures_keep_all_diagnostics() -> eyre::Result<()> {
        let result = deployment_outcome(
            Err(eyre!("deployment failure")),
            Err(eyre!("shutdown failure")),
            Err(eyre!("drain failure")),
        );
        let error = result.expect_err("failure must not become success");
        let message = format!("{error:#}");
        for expected in
            ["deployment failure", "shutdown failure", "drain failure"]
        {
            eyre::ensure!(
                message.contains(expected),
                "lost a concurrent failure: {expected}"
            );
        }
        Ok(())
    }

    #[test]
    fn subprocess_fixture() {
        match std::env::var("DEX_DEPLOY_TEST_CHILD").as_deref() {
            Ok("exit") => std::process::exit(7),
            Ok("sleep") => std::thread::sleep(Duration::from_secs(60)),
            _ => {}
        }
    }

    fn fixture(mode: &str) -> std::io::Result<Child> {
        Command::new(std::env::current_exe()?)
            .args(["--exact", "tests::subprocess_fixture"])
            .env("DEX_DEPLOY_TEST_CHILD", mode)
            .kill_on_drop(true)
            .spawn()
    }

    #[tokio::test]
    async fn shutdown_failure_still_kills_and_reaps_the_child()
    -> eyre::Result<()> {
        let mut child = fixture("sleep")?;
        let missing = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("nonexistent-deploy-signal-program");
        eyre::ensure!(
            !missing.exists(),
            "test signal path unexpectedly exists"
        );
        let outcome = stop_owned_anvil(&mut child, missing.as_os_str()).await;
        let reaped = child.id().is_none();
        if !reaped {
            child.kill().await?;
        }
        eyre::ensure!(
            outcome.is_err(),
            "a failed graceful shutdown must be reported"
        );
        eyre::ensure!(reaped, "shutdown failure leaked its owned child");
        Ok(())
    }

    #[tokio::test]
    async fn shutdown_forces_and_reaps_an_unresponsive_child()
    -> eyre::Result<()> {
        let mut child = fixture("sleep")?;
        let outcome = stop_owned_anvil(&mut child, OsStr::new("true")).await;
        let reaped = child.id().is_none();
        if !reaped {
            child.kill().await?;
        }
        eyre::ensure!(
            outcome.is_err(),
            "forced termination must not announce success"
        );
        eyre::ensure!(reaped, "shutdown timeout leaked its owned child");
        Ok(())
    }

    #[tokio::test]
    async fn shutdown_sends_term_and_reaps_its_child() -> eyre::Result<()> {
        let mut child = fixture("sleep")?;
        stop_owned_anvil(&mut child, OsStr::new("kill")).await?;
        eyre::ensure!(
            child.id().is_none(),
            "graceful shutdown did not reap its child"
        );
        Ok(())
    }

    #[tokio::test]
    async fn shutdown_rejects_an_unexpected_child_exit() -> eyre::Result<()> {
        let mut child = fixture("exit")?;
        let status = child.wait().await?;
        eyre::ensure!(
            status.code() == Some(7),
            "fixture did not exit as expected"
        );
        let outcome = stop_owned_anvil(&mut child, OsStr::new("kill")).await;
        eyre::ensure!(
            outcome.is_err(),
            "unexpected Anvil failure must not announce success"
        );
        Ok(())
    }

    #[tokio::test]
    async fn readiness_rejects_a_different_endpoint() -> eyre::Result<()> {
        let mut lines =
            BufReader::new(&b"Listening on 127.0.0.1:18892\n"[..]).lines();
        let result =
            confirm_owned_listener(&mut lines, "127.0.0.1:18891".parse()?)
                .await;
        eyre::ensure!(
            result.is_err(),
            "another endpoint is not ownership proof"
        );
        Ok(())
    }

    #[tokio::test]
    async fn readiness_rejects_exit_without_listening() -> eyre::Result<()> {
        let mut lines =
            BufReader::new(&b"Error: address already in use\n"[..]).lines();
        let result =
            confirm_owned_listener(&mut lines, "127.0.0.1:18891".parse()?)
                .await;
        eyre::ensure!(
            result.is_err(),
            "an unrelated TCP listener must not count as readiness"
        );
        Ok(())
    }

    #[tokio::test]
    async fn readiness_accepts_the_owned_childs_endpoint() -> eyre::Result<()> {
        let mut lines =
            BufReader::new(&b"Listening on 127.0.0.1:18891\n"[..]).lines();
        confirm_owned_listener(&mut lines, "127.0.0.1:18891".parse()?).await
    }
}
