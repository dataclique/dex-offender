//! Real Surfpool acceptance test; no mock RPC or challenge solution.

use ctf::solana::{SolActor, bootcamp, rpc};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signer::Signer, transaction::Transaction};
use std::{
    net::TcpListener,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    time::Duration,
};

struct Localnet {
    child: Option<Child>,
    directory: PathBuf,
    port: u16,
    ws_port: u16,
}

impl Localnet {
    fn new() -> eyre::Result<Self> {
        let rpc = TcpListener::bind("127.0.0.1:0")?;
        let ws = TcpListener::bind("127.0.0.1:0")?;
        let directory = std::env::temp_dir().join(format!(
            "dex-offender-surfpool-{}-{}",
            std::process::id(),
            rand::random::<u64>(),
        ));
        std::fs::create_dir(&directory)?;
        Ok(Self {
            child: None,
            directory,
            port: rpc.local_addr()?.port(),
            ws_port: ws.local_addr()?.port(),
        })
    }

    fn url(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }

    async fn start(&mut self) -> eyre::Result<()> {
        self.child = Some(
            Command::new("surfpool")
                .current_dir(&self.directory)
                .args([
                    "start",
                    "--offline",
                    "--no-deploy",
                    "--no-tui",
                    "--no-studio",
                    "--host",
                    "127.0.0.1",
                    "--port",
                    &self.port.to_string(),
                    "--ws-port",
                    &self.ws_port.to_string(),
                    "--db",
                    "state.sqlite",
                    "--surfnet-id",
                    "first-level-test",
                ])
                .stdout(Stdio::null())
                .stderr(Stdio::inherit())
                .spawn()?,
        );
        let client =
            RpcClient::new_with_timeout(self.url(), Duration::from_secs(1));
        tokio::time::timeout(Duration::from_secs(30), async {
            loop {
                eyre::ensure!(
                    self.child.as_mut().unwrap().try_wait()?.is_none(),
                    "owned Surfpool exited before readiness"
                );
                if client.get_health().await.is_ok() {
                    return Ok::<(), eyre::Report>(());
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        })
        .await??;
        Ok(())
    }

    fn stop(&mut self) -> eyre::Result<()> {
        if let Some(mut child) = self.child.take() {
            if child.try_wait()?.is_none() {
                child.kill()?;
            }
            child.wait()?;
        }
        Ok(())
    }
}

impl Drop for Localnet {
    fn drop(&mut self) {
        let _ = self.stop();
        let _ = std::fs::remove_dir_all(&self.directory);
    }
}

async fn verify_level_one(
    localnet: &mut Localnet,
    keypair: &Path,
) -> eyre::Result<()> {
    localnet.start().await?;
    let url = localnet.url();
    let offender = rpc::deploy_level_one(&url, keypair).await?;
    let client = offender.rpc_client();
    let target = bootcamp::level_1::Target::from_rpc(client).await?;
    eyre::ensure!(
        !target.check_rpc(client).await?,
        "fresh level must be unsolved"
    );
    let program = client.get_account(&target.program_id).await?;
    eyre::ensure!(program.executable, "first level program must be executable");
    let key_bytes = std::fs::read(keypair)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        eyre::ensure!(
            std::fs::metadata(keypair)?.permissions().mode() & 0o077 == 0,
            "offender keypair must not be readable by other users"
        );
    }

    // Exercise the actual native RpcClient and Signer with an ordinary transfer.
    let recipient = Pubkey::new_unique();
    let transfer = solana_system_interface::instruction::transfer(
        &offender.pubkey(),
        &recipient,
        10_000_000,
    );
    let tx = Transaction::try_new_signed_with_payer(
        &[transfer],
        Some(&offender.pubkey()),
        &[&offender],
        client.get_latest_blockhash().await?,
    )?;
    client.send_and_confirm_transaction(&tx).await?;
    eyre::ensure!(
        client.get_balance(&recipient).await? == 10_000_000,
        "native send failed"
    );

    for program_id in [
        bootcamp::level_2::PROGRAM_ID,
        bootcamp::level_3::PROGRAM_ID,
        bootcamp::level_4::PROGRAM_ID,
        bootcamp::level_5::PROGRAM_ID,
        bootcamp::level_5::VERIFICATION_PROGRAM_ID,
        ctf::solana::hana::PROGRAM_ID,
    ] {
        eyre::ensure!(
            client
                .get_account_with_commitment(
                    &program_id.parse()?,
                    client.commitment()
                )
                .await?
                .value
                .is_none(),
            "a disabled level was deployed"
        );
    }

    // Rerunning deployment must preserve accounts, balances, and identity.
    let again = rpc::deploy_level_one(&url, keypair).await?;
    eyre::ensure!(
        again.pubkey() == offender.pubkey(),
        "identity changed on redeploy"
    );
    eyre::ensure!(
        std::fs::read(keypair)? == key_bytes,
        "keypair file was replaced"
    );
    eyre::ensure!(
        client.get_balance(&recipient).await? == 10_000_000,
        "redeploy reset state"
    );

    localnet.stop()?;
    localnet.start().await?;
    let resumed = SolActor::from_file(&url, keypair)?;
    let client = resumed.rpc_client();
    bootcamp::level_1::Target::from_rpc(client).await?;
    eyre::ensure!(
        client.get_balance(&recipient).await? == 10_000_000,
        "restart lost state"
    );
    eyre::ensure!(
        resumed.pubkey() == offender.pubkey(),
        "restart changed offender"
    );

    // Missing identity is an error, not a silent new keypair on every run.
    eyre::ensure!(
        SolActor::from_file(&url, localnet.directory.join("missing.json"))
            .is_err()
    );
    // Signing cannot silently select any identity other than this actor.
    let message = b"offender identity";
    let signature = resumed.try_sign_message(message)?;
    eyre::ensure!(signature.verify(resumed.pubkey().as_ref(), message));
    eyre::ensure!(!signature.verify(recipient.as_ref(), message));
    Ok(())
}

#[tokio::test]
#[ignore = "starts the Surfpool binary supplied by the Nix dev shell"]
async fn first_level_supports_native_rpc_and_survives_restart()
-> eyre::Result<()> {
    let mut localnet = Localnet::new()?;
    let keypair = localnet.directory.join("offender.json");
    let result = verify_level_one(&mut localnet, &keypair).await;
    localnet.stop()?;
    result
}
