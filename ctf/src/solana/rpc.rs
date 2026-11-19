//! Native Solana RPC access for the persistent local Surfpool.

use crate::solana::ackee::level_1;
use eyre::WrapErr;
use solana_commitment_config::CommitmentConfig;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_rpc_client_api::request::RpcRequest;
use solana_sdk::{
    instruction::Instruction,
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signature, read_keypair_file},
    signer::{Signer, SignerError},
    transaction::Transaction,
};
use std::{
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};

pub const LOCAL_RPC_URL: &str = "http://127.0.0.1:8899";

/// Path of the persistent Ackee setup identities for levels 2-5,
/// created once by deployment and never replaced.
pub fn ackee_actors_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../.solana/ackee-actors.json")
}

pub fn rpc_url() -> eyre::Result<String> {
    match std::env::var("SOLANA_RPC_URL") {
        Ok(url) => Ok(url),
        Err(std::env::VarError::NotPresent) => Ok(LOCAL_RPC_URL.to_owned()),
        Err(error) => Err(error.into()),
    }
}

pub fn keypair_path() -> PathBuf {
    std::env::var_os("SOLANA_KEYPAIR").map(PathBuf::from).unwrap_or_else(|| {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../.solana/offender.json")
    })
}

/// The offender's native RPC client and signing capability. No setup roles.
#[derive(Clone)]
pub struct SolActor {
    /// The actor's signing keypair, exposed for building transactions
    /// that need additional signatures alongside this actor as payer.
    pub keypair: Arc<Keypair>,
    rpc_client: Arc<RpcClient>,
}

impl SolActor {
    pub fn from_file(url: &str, path: impl AsRef<Path>) -> eyre::Result<Self> {
        let path = path.as_ref();
        let keypair = read_keypair_file(path).map_err(|_| {
            eyre::eyre!(
                "cannot load offender keypair at {}; run cargo run -p ctf --bin deploy_levels",
                path.display()
            )
        })?;
        let rpc_client = RpcClient::new_with_commitment(
            url.to_owned(),
            CommitmentConfig::confirmed(),
        );
        Ok(Self {
            keypair: Arc::new(keypair),
            rpc_client: Arc::new(rpc_client),
        })
    }

    pub fn rpc_client(&self) -> &RpcClient {
        &self.rpc_client
    }

    pub fn pubkey(&self) -> Pubkey {
        self.keypair.pubkey()
    }
}

impl Signer for SolActor {
    fn try_pubkey(&self) -> Result<Pubkey, SignerError> {
        self.keypair.try_pubkey()
    }

    fn try_sign_message(
        &self,
        message: &[u8],
    ) -> Result<Signature, SignerError> {
        self.keypair.try_sign_message(message)
    }

    fn is_interactive(&self) -> bool {
        false
    }
}

/// Create the development identity once; never replace an existing keypair.
fn create_offender_keypair(path: &Path) -> eyre::Result<()> {
    if let Some(parent) =
        path.parent().filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)?;
    }
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = match options.open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            return Ok(());
        }
        Err(error) => return Err(error.into()),
    };
    let bytes = serde_json::to_vec(&Keypair::new().to_bytes().as_slice())?;
    file.write_all(&bytes)?;
    file.sync_all()?;
    Ok(())
}

/// Write a prebuilt program at a fixed address using Surfpool's
/// documented writeProgram RPC, or verify an existing deployment.
/// Returns true when the program was written. No upgrade authority is
/// supplied. https://solana.com/docs/tools/surfpool/rpc/cheatcodes#surfnet_writeprogram
pub async fn write_program_if_absent(
    rpc: &RpcClient,
    program_id: &str,
    so_path: &str,
) -> eyre::Result<bool> {
    let id: Pubkey = program_id.parse()?;
    let existing =
        rpc.get_account_with_commitment(&id, rpc.commitment()).await?.value;
    if let Some(account) = existing {
        eyre::ensure!(
            account.executable,
            "program at {program_id} is not executable"
        );
        return Ok(false);
    }
    let elf = std::fs::read(so_path).wrap_err_with(|| {
        format!("cannot read {so_path}; initialize submodules")
    })?;
    let _: serde_json::Value = rpc
        .send(
            RpcRequest::Custom { method: "surfnet_writeProgram" },
            serde_json::json!([program_id, hex::encode(elf), 0]),
        )
        .await?;
    Ok(true)
}

/// Airdrop enough for the address to hold at least `min_lamports`.
pub async fn fund_at_least(
    rpc: &RpcClient,
    address: &Pubkey,
    min_lamports: u64,
) -> eyre::Result<()> {
    let balance = rpc.get_balance(address).await?;
    if balance >= min_lamports {
        return Ok(());
    }
    let blockhash = rpc.get_latest_blockhash().await?;
    let signature =
        rpc.request_airdrop(address, min_lamports - balance).await?;
    rpc.confirm_transaction_with_spinner(
        &signature,
        &blockhash,
        rpc.commitment(),
    )
    .await?;
    Ok(())
}

/// Build, sign, and confirm a transaction on the persistent chain.
/// `payer` signs and pays; `extra_signers` co-sign (e.g. fresh-account
/// keypairs).
pub async fn send_instructions(
    rpc: &RpcClient,
    payer: &Keypair,
    instructions: &[Instruction],
    extra_signers: &[&Keypair],
) -> eyre::Result<()> {
    let mut signers = vec![payer];
    signers.extend_from_slice(extra_signers);
    let message = Message::new(instructions, Some(&payer.pubkey()));
    let transaction =
        Transaction::new(&signers, message, rpc.get_latest_blockhash().await?);
    rpc.send_and_confirm_transaction(&transaction).await?;
    Ok(())
}

/// Initialize only Ancient Vault on the already-running local Surfpool.
/// Existing program state and the offender identity are preserved on reruns.
pub async fn deploy_level_one(
    url: &str,
    keypair_path: impl AsRef<Path>,
) -> eyre::Result<SolActor> {
    let rpc = RpcClient::new_with_commitment(
        url.to_owned(),
        CommitmentConfig::confirmed(),
    );
    rpc.get_health().await.wrap_err_with(|| {
        format!("cannot reach Surfpool at {url}; start it using README.md")
    })?;
    write_program_if_absent(
        &rpc,
        level_1::PROGRAM_ID,
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../lib/solana-auditors-bootcamp/Capture-the-Flag/level-1/target/deploy/level_1.so"
        ),
    )
    .await?;
    level_1::Target::from_rpc(&rpc).await?;
    create_offender_keypair(keypair_path.as_ref())?;
    let offender = SolActor::from_file(url, keypair_path)?;
    if rpc.get_balance(&offender.pubkey()).await? < 1_000_000_000 {
        let blockhash = rpc.get_latest_blockhash().await?;
        let signature =
            rpc.request_airdrop(&offender.pubkey(), 10_000_000_000).await?;
        rpc.confirm_transaction_with_spinner(
            &signature,
            &blockhash,
            rpc.commitment(),
        )
        .await?;
    }
    Ok(offender)
}
