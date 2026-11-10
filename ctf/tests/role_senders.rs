//! Implicit Alloy senders must match the actor, not default to the deployer.

use alloy::{
    network::TransactionBuilder,
    primitives::{Bytes, U256},
    providers::Provider,
    rpc::types::TransactionRequest,
};
use ctf::{Roles, ethereum::ethernaut::lvl04_telephone::Telephone};
use std::{
    net::{SocketAddr, TcpListener},
    process::Stdio,
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{ChildStdout, Command},
    time::timeout,
};

async fn verify_senders(stdout: ChildStdout) -> eyre::Result<SocketAddr> {
    let mut lines = BufReader::new(stdout).lines();
    let endpoint = loop {
        let line = lines.next_line().await?.ok_or_else(|| {
            eyre::eyre!("owned Anvil exited before readiness")
        })?;
        if let Some(endpoint) = line.strip_prefix("Listening on ") {
            break endpoint.to_owned();
        }
    };
    let address: SocketAddr = endpoint.parse()?;
    eyre::ensure!(
        address.ip().is_loopback() && address.port() != 0,
        "invalid owned endpoint"
    );
    let roles = Roles::new(&format!("http://{endpoint}"))?;
    let mut failures = Vec::new();
    for actor in [&roles.deployer, &roles.some_user, &roles.offender] {
        let actor = actor.clone();
        let before =
            roles.provider().get_transaction_count(actor.address()).await?;
        // The same generated deploy API used by practice stubs, without
        // a manual deploy_builder(...).from(...) workaround.
        let contract = Telephone::deploy(actor.provider()).await?;
        let owner = contract.owner().call().await?;
        let receipt = contract.owner().send().await?.get_receipt().await?;

        // A rejected estimate must not consume a cached nonce and wedge
        // the next sequential send. Telephone has no fallback function.
        let rejected = actor
            .provider()
            .send_transaction(
                TransactionRequest::default()
                    .with_to(*contract.address())
                    .with_input(Bytes::from_static(&[0xff])),
            )
            .await;
        eyre::ensure!(rejected.is_err(), "invalid calldata was accepted");
        let transfer = actor
            .provider()
            .send_transaction(
                TransactionRequest::default()
                    .with_to(roles.some_user.address())
                    .with_value(U256::ZERO),
            )
            .await?
            .get_receipt()
            .await?;
        let after =
            roles.provider().get_transaction_count(actor.address()).await?;
        if owner != actor.address()
            || receipt.from != actor.address()
            || transfer.from != actor.address()
            || !receipt.status()
            || !transfer.status()
            || after != before + 3
        {
            failures.push(format!(
                "role {}: owner={owner}, call sender={}, raw sender={}, \
                 nonce {before}->{after}",
                actor.address(),
                receipt.from,
                transfer.from
            ));
        }
    }
    // Setup stays deployer-default; explicit from overrides still select
    // any local role signer; actor providers cannot select another role.
    let contract = Telephone::deploy(roles.provider()).await?;
    eyre::ensure!(
        contract.owner().call().await? == roles.deployer.address(),
        "shared provider changed default sender"
    );
    let unauthorized = Telephone::deploy_builder(roles.offender.provider())
        .from(roles.some_user.address())
        .deploy()
        .await;
    eyre::ensure!(
        unauthorized.is_err(),
        "offender provider must not sign as another role"
    );
    let overridden = Telephone::deploy_builder(roles.provider())
        .from(roles.some_user.address())
        .deploy()
        .await?;
    let overridden = Telephone::new(overridden, roles.provider());
    eyre::ensure!(
        overridden.owner().call().await? == roles.some_user.address(),
        "explicit sender override changed"
    );
    eyre::ensure!(
        failures.is_empty(),
        "wrong implicit role senders: {}",
        failures.join("; ")
    );
    Ok(address)
}

#[tokio::test]
async fn actor_providers_default_to_their_own_sender() -> eyre::Result<()> {
    let mut child = Command::new("anvil")
        .args(["--host", "127.0.0.1", "--port", "0"])
        .kill_on_drop(true)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;
    let outcome = match child.stdout.take() {
        Some(stdout) => {
            timeout(Duration::from_secs(45), verify_senders(stdout))
                .await
                .map_err(eyre::Report::from)
                .and_then(|result| result)
        }
        None => Err(eyre::eyre!("missing owned Anvil stdout")),
    };
    // Cleanup precedes reporting any failure, including startup/RPC errors.
    if child.try_wait()?.is_none() {
        child.start_kill()?;
    }
    timeout(Duration::from_secs(5), child.wait()).await??;
    let address = outcome?;
    eyre::ensure!(
        TcpListener::bind(address).is_ok(),
        "owned Anvil left its port occupied"
    );
    Ok(())
}
