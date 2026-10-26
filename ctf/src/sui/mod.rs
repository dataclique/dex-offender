//! Sui chain core: localnet lifecycle, genesis-funded roles, and level
//! contracts, mirroring the EVM/Solana harness cores.
//!
//! Role keys are generated in-harness (ed25519) and their Sui addresses
//! derived locally (`blake2b256(0x00 ‖ pubkey)`); the network genesis
//! is generated from a template whose `accounts:` section pre-funds
//! exactly those addresses — no faucet service, no CLI keystore, no
//! secret-bearing output. The localnet runs the official `sui` CLI as a
//! child process with everything under `.tmp/sui-localnet` (git-ignored,
//! exclusively owned). Reads go through the official SDK's gRPC client.
//! Set `SUI_LOCALNET_TEST=1` (plus `SUI_BIN` when needed) to run the
//! live lifecycle tests.

use blake2::Blake2bVar;
use blake2::digest::{Update, VariableOutput};
use ed25519_consensus::SigningKey;
use rand::rngs::OsRng;
use std::{
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};
use sui_sdk_types::Address;

/// Default localnet fullnode RPC (JSON-RPC and gRPC multiplexed).
pub const LOCALNET_RPC_URL: &str = "http://127.0.0.1:9000";

/// Gas (MIST) each role receives at genesis.
const ROLE_GAS_MIST: u64 = 30_000_000_000_000;

/// Ways a genesis template can be malformed.
#[derive(Debug, PartialEq)]
pub enum GenesisTemplateError {
    MissingAccountsHeader,
    DuplicateAccountsHeaders(usize),
}

impl std::fmt::Display for GenesisTemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenesisTemplateError::MissingAccountsHeader => {
                write!(f, "genesis template has no accounts: header")
            }
            GenesisTemplateError::DuplicateAccountsHeaders(n) => {
                write!(f, "genesis template has {n} accounts: headers (want 1)")
            }
        }
    }
}

impl std::error::Error for GenesisTemplateError {}

fn port_free(port: u16) -> bool {
    std::net::TcpStream::connect(("127.0.0.1", port)).is_err()
}

/// Derive a Sui address from an ed25519 signing key:
/// `blake2b256(0x00 ‖ pubkey)` — the documented Ed25519 derivation
/// (scheme flag 0x00). Returns a typed error; correctness is exercised
/// by the live lifecycle test (a wrong derivation would read as a zero
/// gRPC balance after genesis pre-funding).
/// Rewrite a compiled module's SELF address to zero — the canonical
/// fresh-publish form. Packages whose upstream authors checked in a
/// `Published.toml` (testnet `original-id`) compile with that concrete
/// self-address embedded, which raw PTB publish rejects
/// (`PUBLISH_ERROR_NON_ZERO_ADDRESS`: modules must self-address 0x0).
/// Only the self module handle's address changes; code and external
/// dependencies (0x1/0x2/…) are untouched.
pub fn zero_self_address(module_bytes: &[u8]) -> eyre::Result<Vec<u8>> {
    use move_binary_format::internals::ModuleIndex;
    let mut module =
        move_binary_format::CompiledModule::deserialize_with_defaults(
            module_bytes,
        )
        .map_err(|e| eyre::eyre!("deserialize move module: {e:?}"))?;
    // self_handle_idx() always returns the first module handle
    let self_idx = module.self_handle_idx();
    let addr_idx =
        module.module_handles[self_idx.into_index()].address.into_index();
    module.address_identifiers[addr_idx] =
        move_core_types::account_address::AccountAddress::ZERO;
    let mut out = Vec::with_capacity(module_bytes.len());
    module
        .serialize(&mut out)
        .map_err(|e| eyre::eyre!("serialize move module: {e}"))?;
    Ok(out)
}

fn derive_address(key: &SigningKey) -> eyre::Result<Address> {
    let mut hasher =
        Blake2bVar::new(32).map_err(|e| eyre::eyre!("blake2b init: {e:?}"))?;
    hasher.update(&[0x00]);
    hasher.update(&key.verification_key().to_bytes());
    let mut out = [0u8; 32];
    hasher
        .finalize_variable(&mut out)
        .map_err(|e| eyre::eyre!("blake2b finalize: {e:?}"))?;
    Ok(Address::new(out))
}

/// Convert whole SUI to MIST at the production boundary used by funding
/// helpers. Overflow surfaces as a typed error.
pub fn sui_to_mist(amount_sui: u64) -> eyre::Result<u64> {
    amount_sui
        .checked_mul(1_000_000_000)
        .ok_or_else(|| eyre::eyre!("amount {amount_sui} SUI overflows MIST"))
}

/// Terminate the owned localnet child, returning an error when the
/// process cannot be confirmed dead. Scratch deletion is the CALLER's
/// job and must only happen after this returns Ok — a failed stop
/// preserves artifacts.
fn stop_child(child: &mut Child, reason: &str) -> eyre::Result<()> {
    child.kill().map_err(|e| {
        eyre::eyre!("{reason}: kill failed ({e}); scratch preserved")
    })?;
    child.wait().map_err(|e| {
        eyre::eyre!("{reason}: wait failed ({e}); scratch preserved")
    })?;
    Ok(())
}

/// The `sui` binary (dev-shell provided; overridable via SUI_BIN).
fn sui_command() -> Command {
    let bin = std::env::var("SUI_BIN").unwrap_or_else(|_| "sui".into());
    Command::new(bin)
}

/// Absolute scratch path anchored at the workspace root
/// (cwd-independent).
fn scratch_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join(".tmp/sui-localnet")
}

/// Insert pre-funded accounts into a genesis template's `accounts:`
/// section (right after the header line, preserving the dev account).
/// Malformed templates (missing or duplicate `accounts:` headers)
/// return a typed error instead of silently mis-funding.
fn inject_accounts(
    template: &str,
    accounts: &[Address],
) -> Result<String, GenesisTemplateError> {
    let headers =
        template.lines().filter(|l| l.trim_end() == "accounts:").count();
    match headers {
        1 => {}
        0 => return Err(GenesisTemplateError::MissingAccountsHeader),
        n => return Err(GenesisTemplateError::DuplicateAccountsHeaders(n)),
    }
    let mut out = String::with_capacity(template.len() + accounts.len() * 96);
    for line in template.lines() {
        out.push_str(line);
        out.push('\n');
        if line.trim_end() == "accounts:" {
            for address in accounts {
                out.push_str(&format!(
                    "  - address: \"{address}\"\n    gas_amounts:\n      - {ROLE_GAS_MIST}\n"
                ));
            }
        }
    }
    Ok(out)
}

/// One role's handle: a signing key (with its raw seed for the CLI
/// signer wrapper), its derived address, and a gRPC client. Keys stay
/// in-harness; nothing secret is printed or persisted beyond this
/// process.
pub struct SuiActor {
    pub address: Address,
    pub signing_key: SigningKey,
    pub seed: [u8; 32],
    client: sui_rpc::Client,
}

impl SuiActor {
    /// Total SUI gas balance (MIST) for this role, via gRPC.
    pub async fn gas_balance(&self) -> eyre::Result<u128> {
        use sui_rpc::proto::sui::rpc::v2::GetBalanceRequest;
        let mut state = self.client.clone().state_client();
        let mut request = GetBalanceRequest::const_default();
        request.owner = Some(self.address.to_string());
        request.coin_type = Some("0x2::sui::SUI".to_string());
        let response = state.get_balance(request).await?;
        let balance = response
            .into_inner()
            .balance
            .and_then(|b| b.balance)
            .ok_or_else(|| eyre::eyre!("no balance returned"))?;
        Ok(u128::from(balance))
    }
}

/// Three roles mirroring the EVM/Solana harnesses.
pub struct SuiRoles {
    pub deployer: SuiActor,
    pub some_user: SuiActor,
    pub offender: SuiActor,
}

/// Result of a successful publish transaction.
#[derive(Debug, Clone)]
pub struct PublishOutcome {
    /// The immutable package object id.
    pub package: String,
    /// Every object created by the transaction (UpgradeCap, package,
    /// and any module `init` state), with their owner kinds
    /// (proto `owner::OwnerKind` values).
    pub created: Vec<CreatedObject>,
}

/// One created object from publish effects.
#[derive(Debug, Clone)]
pub struct CreatedObject {
    pub id: String,
    pub owner_kind: i32,
}

impl PublishOutcome {
    /// The first created object owned as `kind` (proto OwnerKind),
    /// excluding the package itself.
    pub fn created_with_owner(&self, kind: i32) -> Option<&str> {
        self.created
            .iter()
            .find(|object| {
                object.owner_kind == kind && object.id != self.package
            })
            .map(|object| object.id.as_str())
    }
}

/// A running local Sui network owned by this handle.
pub struct SuiLocalnet {
    child: Child,
    dir: PathBuf,
    client: sui_rpc::Client,
    torn_down: bool,
}

impl SuiLocalnet {
    /// Generate three in-harness role keys, derive their addresses,
    /// generate a genesis template pre-funding them, run genesis +
    /// localnet, wait for RPC readiness, and confirm every role balance
    /// is positive via gRPC.
    pub async fn start_with_roles() -> eyre::Result<(Self, SuiRoles)> {
        for port in [9000, 9123] {
            if !port_free(port) {
                return Err(eyre::eyre!("port {port} already in use"));
            }
        }

        let dir = scratch_path();
        // Exclusive ownership: the scratch must not exist at all — a
        // pre-existing path (even empty, even a symlink) is not ours to
        // regenerate or recursively delete.
        if dir.symlink_metadata().is_ok() {
            return Err(eyre::eyre!(
                "scratch {} already exists; remove it first",
                dir.display()
            ));
        }
        std::fs::create_dir(&dir)?;
        let cleanup_dir = || {
            let _ = std::fs::remove_dir_all(&dir);
        };

        let scratch = dir
            .to_str()
            .ok_or_else(|| {
                eyre::eyre!("scratch path {} is not valid utf-8", dir.display())
            })?
            .to_string();

        // three role keys, addresses derived in-harness
        let mut keys: Vec<(SigningKey, [u8; 32])> = (0..3)
            .map(|_| {
                let mut seed = [0u8; 32];
                rand::RngCore::fill_bytes(&mut OsRng, &mut seed);
                (SigningKey::from(seed), seed)
            })
            .collect();
        let mut addresses = Vec::with_capacity(3);
        for (key, _) in &keys {
            addresses.push(derive_address(key)?);
        }

        // genesis template pre-funding exactly the role addresses; the
        // template lives OUTSIDE the scratch dir so `genesis -f` never
        // wipes it, and is created fresh per run (exclusively ours).
        let template_path = Self::template_path();
        if template_path.symlink_metadata().is_ok() {
            cleanup_dir();
            return Err(eyre::eyre!(
                "template {} already exists; remove it first",
                template_path.display()
            ));
        }
        // full cleanup from here on: template + scratch together
        let cleanup_all = || {
            let _ = std::fs::remove_file(&template_path);
            let _ = std::fs::remove_dir_all(&dir);
        };
        let template_str = template_path
            .to_str()
            .ok_or_else(|| eyre::eyre!("template path is not valid utf-8"))?
            .to_string();
        let write = sui_command()
            .args(["genesis", "--write-config", template_str.as_str()])
            .output()?;
        if !write.status.success() {
            cleanup_all();
            return Err(eyre::eyre!(
                "sui genesis --write-config failed: {}",
                String::from_utf8_lossy(&write.stderr)
            ));
        }
        let template = std::fs::read_to_string(&template_path)?;
        std::fs::write(
            &template_path,
            inject_accounts(&template, &addresses)
                .map_err(eyre::Report::from)?,
        )?;

        let genesis = sui_command()
            .args([
                "genesis",
                "-f",
                "--from-config",
                template_str.as_str(),
                "--working-dir",
                scratch.as_str(),
            ])
            .output()?;
        if !genesis.status.success() {
            // genesis failed before any child started; scratch is ours
            cleanup_all();
            return Err(eyre::eyre!(
                "sui genesis failed: {}{}",
                String::from_utf8_lossy(&genesis.stdout),
                String::from_utf8_lossy(&genesis.stderr)
            ));
        }

        let mut child = sui_command()
            .args(["start", "--network.config", scratch.as_str()])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        // readiness: the gRPC client can read a checkpoint
        let deadline = Instant::now() + Duration::from_secs(180);
        let client = loop {
            if Instant::now() > deadline {
                let reason = "localnet rpc never became ready";
                // confirmed termination FIRST; scratch survives a failed stop
                let stop = stop_child(&mut child, reason);
                if stop.is_ok() {
                    cleanup_all();
                }
                stop?;
                return Err(eyre::eyre!("{reason}"));
            }
            if let Ok(client) = sui_rpc::Client::new(LOCALNET_RPC_URL) {
                let mut ledger = client.clone().ledger_client();
                if ledger
                    .get_checkpoint(
                        sui_rpc::proto::sui::rpc::v2::GetCheckpointRequest::const_default(),
                    )
                    .await
                    .is_ok()
                {
                    break client;
                }
            }
            tokio::time::sleep(Duration::from_millis(1500)).await;
        };

        let localnet = Self { child, dir, client, torn_down: false };

        let (deployer_key, deployer_seed) = keys.remove(0);
        let (user_key, user_seed) = keys.remove(0);
        let (offender_key, offender_seed) = keys.remove(0);
        let roles = SuiRoles {
            deployer: SuiActor {
                address: addresses[0],
                signing_key: deployer_key,
                seed: deployer_seed,
                client: localnet.client(),
            },
            some_user: SuiActor {
                address: addresses[1],
                signing_key: user_key,
                seed: user_seed,
                client: localnet.client(),
            },
            offender: SuiActor {
                address: addresses[2],
                signing_key: offender_key,
                seed: offender_seed,
                client: localnet.client(),
            },
        };

        // funding verification: every derived address must hold its
        // genesis gas (this is the end-to-end derivation check)
        let deadline = Instant::now() + Duration::from_secs(60);
        loop {
            let mut all_funded = true;
            for actor in [&roles.deployer, &roles.some_user, &roles.offender] {
                if actor.gas_balance().await? == 0 {
                    all_funded = false;
                }
            }
            if all_funded {
                break;
            }
            if Instant::now() > deadline {
                let _ = localnet.teardown().await;
                return Err(eyre::eyre!(
                    "role genesis funding not observed within 60s (derivation or genesis failed)"
                ));
            }
            tokio::time::sleep(Duration::from_millis(1500)).await;
        }

        Ok((localnet, roles))
    }

    /// Publish compiled Move modules (bytecode) as a package from the
    /// given role's key. Signing uses the in-harness key through the
    /// official sui-crypto signer. Returns the package id plus every
    /// object created by the publish transaction and any module `init`
    /// (e.g. shared challenge state), with ownership kinds.
    pub async fn publish(
        &self,
        role: &SuiActor,
        modules: Vec<Vec<u8>>,
        dependencies: Vec<Address>,
    ) -> eyre::Result<PublishOutcome> {
        use sui_rpc::proto::sui::rpc::v2::changed_object::IdOperation;
        use sui_rpc::proto::sui::rpc::v2::owner::OwnerKind;
        use sui_rpc::proto::sui::rpc::v2::{
            ExecuteTransactionRequest, Transaction as ProtoTransaction,
        };
        use sui_transaction_builder::TransactionBuilder;

        let mut client = self.client();
        let mut builder = TransactionBuilder::new();
        let cap = builder.publish(modules, dependencies);
        // the publish result (UpgradeCap) must be consumed: transfer it
        // to the publishing role (the standard publish pattern)
        let recipient = builder.pure(&role.address);
        builder.transfer_objects(vec![cap], recipient);
        builder.set_sender(role.address);
        let transaction = builder
            .build(&mut client)
            .await
            .map_err(|e| eyre::eyre!("build publish transaction: {e}"))?;

        // sign with the in-harness ed25519 key: construct the CLI-style
        // `flag ‖ key` base64 encoding (flag 0x00 = ed25519) in memory
        let mut flagged = Vec::with_capacity(33);
        flagged.push(0x00);
        flagged.extend_from_slice(&role.seed);
        use base64::Engine as _;
        let encoded =
            base64::engine::general_purpose::STANDARD.encode(&flagged);
        let keypair = sui_crypto::simple::SimpleKeypair::from_base64(&encoded)
            .map_err(|e| eyre::eyre!("construct signer from role key: {e}"))?;
        use sui_crypto::SuiSigner as _;
        let signature = keypair
            .sign_transaction(&transaction)
            .map_err(|e| eyre::eyre!("sign publish transaction: {e}"))?;

        let mut execution = client.execution_client();
        // request the effect fields we read: whole-message masks for
        // status and changed_objects (the default mask omits them)
        use sui_rpc::field::{FieldMask, FieldMaskUtil};
        let mask = FieldMask::from_paths([
            "effects.status".to_string(),
            "effects.changed_objects".to_string(),
        ]);
        let mut request = ExecuteTransactionRequest::const_default()
            .with_transaction(ProtoTransaction::from(transaction))
            .with_signatures(vec![
                sui_rpc::proto::sui::rpc::v2::UserSignature::from(signature),
            ]);
        request.read_mask = Some(mask);
        let response =
            execution.execute_transaction(request).await?.into_inner();

        // the created immutable object is the package
        let executed = response.transaction.ok_or_else(|| {
            eyre::eyre!("execution response missing transaction")
        })?;
        let effects = executed
            .effects
            .ok_or_else(|| eyre::eyre!("execution response missing effects"))?;
        let status = effects
            .status
            .ok_or_else(|| eyre::eyre!("effects missing status"))?;
        if let Some(err) = status.error {
            return Err(eyre::eyre!("publish failed: {}", err));
        }
        // created objects: the UpgradeCap (address-owned), the package
        // (immutable → not address-owned), plus anything a module
        // `init` created (e.g. shared challenge state)
        let created: Vec<CreatedObject> = effects
            .changed_objects
            .iter()
            .filter(|changed| {
                changed.id_operation == Some(IdOperation::Created as i32)
            })
            .filter_map(|changed| {
                let id = changed.object_id.clone()?;
                Some(CreatedObject {
                    id,
                    owner_kind: changed
                        .output_owner
                        .as_ref()
                        .and_then(|owner| owner.kind)
                        .unwrap_or_default(),
                })
            })
            .collect();
        let package = created
            .iter()
            .find(|object| object.owner_kind != OwnerKind::Address as i32)
            .map(|object| object.id.clone())
            .ok_or_else(|| {
                eyre::eyre!(
                    "publish created no package object; created: {created:?}"
                )
            })?;
        Ok(PublishOutcome { package, created })
    }

    /// The shared gRPC client (cloneable).
    pub fn client(&self) -> sui_rpc::Client {
        self.client.clone()
    }

    /// The localnet scratch directory (removed on confirmed teardown).
    pub fn scratch_dir(&self) -> &Path {
        &self.dir
    }

    /// The per-run genesis template path (removed with the network).
    fn template_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../.tmp/sui-genesis-template.ini")
    }

    /// Stop the network (confirmed) and remove the scratch directory
    /// plus the per-run genesis template.
    pub async fn teardown(mut self) -> eyre::Result<()> {
        if self.torn_down {
            return Ok(());
        }
        self.torn_down = true;
        stop_child(&mut self.child, "localnet teardown")?;
        // the OS needs a moment to release the port; callers assert it
        tokio::time::sleep(Duration::from_secs(2)).await;
        std::fs::remove_dir_all(&self.dir)?;
        let _ = std::fs::remove_file(Self::template_path());
        Ok(())
    }
}

impl Drop for SuiLocalnet {
    fn drop(&mut self) {
        if !self.torn_down {
            self.torn_down = true;
            // best-effort sync mirror of teardown; errors preserve scratch
            if stop_child(&mut self.child, "localnet drop").is_ok()
                && self.dir.exists()
            {
                let _ = std::fs::remove_dir_all(&self.dir);
                let _ = std::fs::remove_file(Self::template_path());
            }
        }
    }
}

// Re-export the chain-agnostic Level/Exploit contracts so Sui level
// modules use the same shape as the other chains.
pub use crate::level::{Exploit, Level};

pub mod lets_ctf;
pub mod mysten_ctf;

/// Deployed-Sui-level state mirrored into `ctfs.json` under `"sui"`.
#[derive(
    Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize,
)]
pub struct SuiBoard {
    /// Smoke fixture package id (proves the publish round-trip).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smoke: Option<SmokeTarget>,
    /// lets-ctf check_in level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lets_ctf_check_in: Option<lets_ctf::check_in::Target>,
    /// mysten-ctf sabotage_arena level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mysten_sabotage_arena: Option<mysten_ctf::sabotage_arena::Target>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SmokeTarget {
    pub package: Address,
}

impl SuiBoard {
    pub fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.sui)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEMPLATE: &str = "parameters:\n  a: 1\naccounts:\n  - address: \"0x100fbf7e48f5ee852cae131982e2ea9c40bfe7f25ba078ac1b5e70cedf3b7d56\"\n    gas_amounts:\n      - 30000000000000000\n";

    #[test]
    fn fund_amount_overflow_is_a_typed_error() {
        let err = sui_to_mist(u64::MAX).expect_err("must fail on overflow");
        assert!(
            err.to_string().contains("overflows MIST"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn converts_whole_sui_to_mist() {
        assert_eq!(sui_to_mist(1).unwrap(), 1_000_000_000);
        assert_eq!(sui_to_mist(0).unwrap(), 0);
    }

    #[test]
    fn derive_address_is_deterministic_and_key_dependent() {
        let seed_a = [7u8; 32];
        let seed_b = [8u8; 32];
        let a1 = derive_address(&SigningKey::from(seed_a)).unwrap();
        let a2 = derive_address(&SigningKey::from(seed_a)).unwrap();
        let b = derive_address(&SigningKey::from(seed_b)).unwrap();
        assert_eq!(a1, a2, "same key must derive the same address");
        assert_ne!(a1, b, "different keys must derive different addresses");
    }

    #[test]
    fn inject_accounts_keeps_dev_account_and_adds_roles() {
        let addr1 = Address::from_static(
            "0x0000000000000000000000000000000000000000000000000000000000000001",
        );
        let addr2 = Address::from_static(
            "0x0000000000000000000000000000000000000000000000000000000000000002",
        );
        let out =
            inject_accounts(TEMPLATE, &[addr1, addr2]).expect("valid template");
        assert!(out.contains("parameters:"), "template prefix preserved");
        assert!(out.contains("0x100fbf7e"), "dev account retained");
        assert!(out.contains(&addr1.to_string()), "role 1 injected");
        assert!(out.contains(&addr2.to_string()), "role 2 injected");
        let header = out.find("accounts:").expect("header");
        let role1 = out.find(&addr1.to_string()).expect("role 1");
        let dev = out.find("0x100fbf7e").expect("dev");
        assert!(header < role1 && role1 < dev, "injection order");
    }

    #[test]
    fn inject_accounts_rejects_missing_header() {
        let err = inject_accounts("parameters:\n  a: 1\n", &[])
            .expect_err("must fail");
        assert_eq!(err, GenesisTemplateError::MissingAccountsHeader);
        assert!(err.to_string().contains("no accounts: header"));
    }

    #[test]
    fn inject_accounts_rejects_duplicate_headers() {
        let malformed = format!("{TEMPLATE}accounts:\n");
        let err = inject_accounts(&malformed, &[]).expect_err("must fail");
        assert_eq!(err, GenesisTemplateError::DuplicateAccountsHeaders(2));
        assert!(err.to_string().contains("2 accounts: headers"));
    }
}
