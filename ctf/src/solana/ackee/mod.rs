//! Ackee Solana Auditors Bootcamp CTF levels 1–5 on the persistent
//! Surfpool localnet.
//!
//! Levels 2–5 hold persistent setup identities (created once by
//! `deploy_levels -- solana`, never replaced) because their evaluation
//! and join instructions must be signed by setup-side keypairs across
//! runs. Level 1 needs none: its vault PDA derives from the program id.

pub mod level_1;
pub mod level_2;
pub mod level_3;
pub mod level_4;
pub mod level_5;

use eyre::WrapErr;
use serde::{Deserialize, Serialize};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use std::{fs::OpenOptions, io::Write, path::Path};

/// A persisted secret key, stored as its 32-byte seed encoding.
#[derive(Serialize, Deserialize)]
pub struct Keyfile(Vec<u8>);

impl Keyfile {
    fn generate() -> Self {
        Self(Keypair::new().to_bytes()[..32].to_vec())
    }

    fn keypair(&self) -> eyre::Result<Keypair> {
        let seed: [u8; 32] = self
            .0
            .as_slice()
            .try_into()
            .map_err(|_| eyre::eyre!("stored actor key is malformed"))?;
        Ok(Keypair::new_from_array(seed))
    }
}

/// Setup identities for Ackee levels 2–5, persisted once by deployment.
/// Practice progress is bound to these addresses: the file is created
/// with `create_new` semantics and never replaced.
#[derive(Serialize, Deserialize)]
pub struct AckeeActors {
    pub level_2: level_2::Actors,
    pub level_3: level_3::Actors,
    pub level_4: level_4::Actors,
    pub level_5: level_5::Actors,
}

impl AckeeActors {
    /// Fresh random identities for a first deployment.
    pub fn generate() -> Self {
        Self {
            level_2: level_2::Actors::generate(),
            level_3: level_3::Actors::generate(),
            level_4: level_4::Actors::generate(),
            level_5: level_5::Actors::generate(),
        }
    }

    /// Load persisted identities, creating the file on first use. An
    /// existing file always wins over regeneration.
    pub fn load_or_create(path: &Path) -> eyre::Result<Self> {
        if path.exists()
            && let Ok(text) = std::fs::read_to_string(path)
        {
            return parse(&text, path);
        }
        let actors = Self::generate();
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
            // The file appeared meanwhile (or a first read failed for
            // another reason): the persisted identities always win.
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                let text =
                    std::fs::read_to_string(path).wrap_err_with(|| {
                        format!("cannot read {}", path.display())
                    })?;
                return parse(&text, path);
            }
            Err(error) => {
                return Err(error).wrap_err_with(|| {
                    format!("cannot create {}", path.display())
                });
            }
        };
        file.write_all(&serde_json::to_vec(&actors)?)?;
        file.sync_all()?;
        Ok(actors)
    }
}

fn parse(text: &str, path: &Path) -> eyre::Result<AckeeActors> {
    serde_json::from_str(text)
        .wrap_err_with(|| format!("cannot parse {}", path.display()))
}

/// Fail with actionable context when a level program is not deployed.
pub(crate) async fn require_program(
    rpc: &RpcClient,
    program_id: &str,
    level: &str,
) -> eyre::Result<Pubkey> {
    let id = program_id.parse()?;
    let account =
        rpc.get_account_with_commitment(&id, rpc.commitment()).await?.value;
    eyre::ensure!(
        account.is_some_and(|account| account.executable),
        "{level} is not deployed; run cargo run -p ctf --bin deploy_levels -- solana"
    );
    Ok(id)
}

/// Whether the account exists and is owned by `owner`.
pub(crate) async fn account_owned_by(
    rpc: &RpcClient,
    address: &Pubkey,
    owner: &Pubkey,
) -> eyre::Result<bool> {
    let account =
        rpc.get_account_with_commitment(address, rpc.commitment()).await?.value;
    Ok(account.is_some_and(|account| account.owner == *owner))
}

/// Deploy programs and run setup for Ackee levels 2–5 against the
/// persistent chain. Every level is attempted; failures are reported
/// together so one blocked level does not hide the rest.
pub async fn deploy_levels_two_to_five(
    rpc: &RpcClient,
    offender: &Keypair,
    actors: &AckeeActors,
) -> eyre::Result<()> {
    let mut failures: Vec<&'static str> = Vec::new();
    for (name, result) in [
        (
            "level 2",
            level_2::Target::set_up(rpc, &actors.level_2, offender).await,
        ),
        (
            "level 3",
            level_3::Target::set_up(rpc, &actors.level_3, offender).await,
        ),
        (
            "level 4",
            level_4::Target::set_up(rpc, &actors.level_4, offender).await,
        ),
        (
            "level 5",
            level_5::Target::set_up(rpc, &actors.level_5, offender).await,
        ),
    ] {
        match result {
            Ok(()) => println!("{name} ready"),
            Err(error) => {
                println!("{name} setup failed: {error:#}");
                failures.push(name);
            }
        }
    }
    if failures.is_empty() {
        Ok(())
    } else {
        eyre::bail!("incomplete Ackee setup: {}", failures.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::signature::Signer;
    use std::path::PathBuf;

    fn scratch(name: &str) -> PathBuf {
        let path = std::env::temp_dir()
            .join(format!("dex-offender-actors-{}-{name}", std::process::id()));
        let _ = std::fs::remove_file(&path);
        path
    }

    #[test]
    fn generated_identities_round_trip_through_the_file() -> eyre::Result<()> {
        let path = scratch("roundtrip.json");
        let created = AckeeActors::load_or_create(&path)?;
        let reloaded = AckeeActors::load_or_create(&path)?;
        assert_eq!(
            created.level_2.user1.keypair()?.pubkey(),
            reloaded.level_2.user1.keypair()?.pubkey()
        );
        assert_eq!(
            created.level_5.escrow_token.keypair()?.pubkey(),
            reloaded.level_5.escrow_token.keypair()?.pubkey()
        );
        let _ = std::fs::remove_file(&path);
        Ok(())
    }

    #[test]
    fn an_existing_file_is_never_replaced() -> eyre::Result<()> {
        let path = scratch("keep.json");
        let first = AckeeActors::load_or_create(&path)?;
        let persisted = std::fs::read_to_string(&path)?;
        let second = AckeeActors::load_or_create(&path)?;
        assert_eq!(persisted, std::fs::read_to_string(&path)?);
        assert_eq!(
            first.level_3.creator.keypair()?.pubkey(),
            second.level_3.creator.keypair()?.pubkey()
        );
        let _ = std::fs::remove_file(&path);
        Ok(())
    }

    #[test]
    fn a_corrupt_file_is_rejected_not_regenerated() -> eyre::Result<()> {
        let path = scratch("corrupt.json");
        std::fs::write(&path, "not json")?;
        let outcome = AckeeActors::load_or_create(&path);
        assert!(outcome.is_err(), "corrupt identities must fail loudly");
        assert_eq!(std::fs::read_to_string(&path)?, "not json");
        let _ = std::fs::remove_file(&path);
        Ok(())
    }
}
