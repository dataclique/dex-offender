#![cfg(unix)]
//! Exercise the deployment binary on its own port, never the player's node.

use std::{net::TcpListener, process::Stdio, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, BufReader},
    process::{Child, Command},
    task::JoinHandle,
    time::timeout,
};

async fn relay(reader: impl AsyncRead + Unpin) -> std::io::Result<String> {
    let mut output = String::new();
    let mut lines = BufReader::new(reader).lines();
    while let Some(line) = lines.next_line().await? {
        println!("{line}");
        output.push_str(&line);
        output.push('\n');
    }
    Ok(output)
}

async fn collect(
    task: &mut JoinHandle<std::io::Result<String>>,
) -> eyre::Result<String> {
    match timeout(Duration::from_secs(3), &mut *task).await {
        Ok(result) => Ok(result??),
        Err(error) => {
            task.abort();
            Err(error.into())
        }
    }
}

async fn terminate_owned_group(
    child: &mut Child,
    group: u32,
) -> eyre::Result<()> {
    let group = i32::try_from(group)?;
    eyre::ensure!(group > 1, "invalid owned test process group");
    let killed = timeout(
        Duration::from_secs(3),
        Command::new("kill")
            .kill_on_drop(true)
            .args(["-KILL", "--", &format!("-{group}")])
            .status(),
    )
    .await??;
    eyre::ensure!(
        killed.success(),
        "could not stop owned deployment test group {group}"
    );
    timeout(Duration::from_secs(3), child.wait()).await??;
    Ok(())
}

#[tokio::test]
async fn invalid_deployment_ports_fail_before_starting_anvil()
-> eyre::Result<()> {
    for port in ["", "0", "65536", "not-a-port"] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_deploy_levels"));
        command.env("DEPLOY_PORT", port).kill_on_drop(true);
        let output =
            timeout(Duration::from_secs(3), command.output()).await??;
        eyre::ensure!(
            !output.status.success(),
            "invalid deployment port was accepted"
        );
        eyre::ensure!(
            !String::from_utf8_lossy(&output.stdout).contains("Spawning"),
            "invalid port started Anvil"
        );
        eyre::ensure!(
            String::from_utf8_lossy(&output.stderr).contains("DEPLOY_PORT"),
            "missing deployment-port diagnostic"
        );
    }
    Ok(())
}

#[tokio::test]
async fn deployment_command_finishes_and_saves_state() -> eyre::Result<()> {
    let reservation = TcpListener::bind(("127.0.0.1", 0))?;
    let port = reservation.local_addr()?.port();
    let root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let scratch =
        root.join(".tmp/deploy-tests").join(std::process::id().to_string());
    std::fs::create_dir_all(scratch.parent().unwrap())?;
    std::fs::create_dir(&scratch)?;
    drop(reservation);

    let mut child = Command::new(env!("CARGO_BIN_EXE_deploy_levels"))
        .current_dir(&scratch)
        .env("DEPLOY_PORT", port.to_string())
        .env_remove("FORK_URL")
        .env_remove("FORK_BLOCK")
        .process_group(0)
        .kill_on_drop(true)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let group =
        child.id().ok_or_else(|| eyre::eyre!("owned command has no PID"))?;
    let mut stdout_task = tokio::spawn(relay(child.stdout.take().unwrap()));
    let mut stderr_task = tokio::spawn(relay(child.stderr.take().unwrap()));
    let mut group_finished = false;

    let outcome: eyre::Result<()> = async {
        let status = timeout(Duration::from_secs(330), child.wait()).await??;
        let stdout = collect(&mut stdout_task).await?;
        let stderr = collect(&mut stderr_task).await?;
        let saved = scratch.join("ctfs.json").is_file()
            && scratch.join("state.json").is_file();
        let freed = TcpListener::bind(("127.0.0.1", port)).is_ok();
        // The leader is reaped, both inherited output pipes are closed, and
        // its Anvil no longer holds the port. Do not signal a retired group.
        group_finished = freed;

        eyre::ensure!(status.success(), "deployment command failed: {stderr}");
        eyre::ensure!(
            saved,
            "deployment command did not save both state files"
        );
        eyre::ensure!(
            stdout.contains("Deployment complete:"),
            "deployment command does not announce completion"
        );
        eyre::ensure!(freed, "deployment command left its node running");
        Ok(())
    }
    .await;

    let cleanup = if group_finished {
        Ok(())
    } else {
        terminate_owned_group(&mut child, group).await
    };
    stdout_task.abort();
    stderr_task.abort();
    if let Err(error) = cleanup {
        return match outcome {
            Err(primary) => Err(primary.wrap_err(format!(
                "owned group cleanup failed: {error}; preserve {}",
                scratch.display()
            ))),
            Ok(()) => Err(error),
        };
    }
    outcome.map_err(|error| {
        error.wrap_err(format!("deployment evidence: {}", scratch.display()))
    })?;
    std::fs::remove_dir_all(&scratch)?;
    Ok(())
}
