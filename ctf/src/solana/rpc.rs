//! Native Solana RPC access for the persistent local Surfpool.

use crate::solana::bootcamp::level_1;
use eyre::WrapErr;
use solana_commitment_config::CommitmentConfig;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_rpc_client_api::request::RpcRequest;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature, read_keypair_file},
    signer::{Signer, SignerError},
};
use std::{
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};

pub const LOCAL_RPC_URL: &str = "http://127.0.0.1:8899";

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
    keypair: Arc<Keypair>,
    rpc_client: Arc<RpcClient>,
}

impl SolActor {
    pub fn from_file(url: &str, path: impl AsRef<Path>) -> eyre::Result<Self> {
        let path = path.as_ref();
        let keypair = read_keypair_file(path).map_err(|_| {
            eyre::eyre!(
                "cannot load offender keypair at {}; run cargo run -p ctf --bin deploy_solana_levels",
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
    let program_id: Pubkey = level_1::PROGRAM_ID.parse()?;
    let existing = rpc
        .get_account_with_commitment(&program_id, rpc.commitment())
        .await?
        .value;
    if let Some(account) = existing {
        eyre::ensure!(
            account.executable,
            "Ancient Vault address is not executable"
        );
    } else {
        let elf = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../lib/solana-auditors-bootcamp/Capture-the-Flag/level-1/target/deploy/level_1.so"
        )).wrap_err("cannot read the pinned Ancient Vault program; initialize submodules")?;
        // Surfpool's documented writeProgram RPC creates the fixed-address
        // program and program-data accounts. No upgrade authority is supplied.
        // https://solana.com/docs/tools/surfpool/rpc/cheatcodes#surfnet_writeprogram
        let _: serde_json::Value = rpc
            .send(
                RpcRequest::Custom { method: "surfnet_writeProgram" },
                serde_json::json!([level_1::PROGRAM_ID, hex::encode(elf), 0]),
            )
            .await?;
    }
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
