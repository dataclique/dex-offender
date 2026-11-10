//! Solana CTF harness core using LiteSVM for in-process program execution.
//!
//! Mirrors the EVM harness in [`crate::roles`] and [`crate::level`]:
//! a [`Roles`] with three actors, a chain abstraction, and the same
//! Level/Exploit traits (re-exported from [`crate::level`]).

use solana_sdk::instruction::Instruction;
use solana_sdk::message::Message;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signature};
use solana_sdk::signer::{Signer, SignerError};
use solana_sdk::transaction::Transaction;
use std::sync::{Arc, Mutex};

pub use litesvm;

pub mod bootcamp;
pub mod hana;

/// Abstraction over the Solana execution environment.
/// Default impl wraps `litesvm::LiteSVM` (in-process, no binary needed).
pub trait SolChain: Send + Sync {
    fn deploy_program(&self, id: Pubkey, elf: &[u8]) -> eyre::Result<()>;
    /// Deploy with an explicit loader — legacy (solana 1.9-era) programs
    /// require `bpf_loader::id()` (BPFLoader2), not the upgradeable one.
    fn deploy_program_with_loader(
        &self,
        id: Pubkey,
        elf: &[u8],
        loader: Pubkey,
    ) -> eyre::Result<()>;
    fn deploy_program_from_file(
        &self,
        id: Pubkey,
        path: &str,
    ) -> eyre::Result<()>;
    fn send(
        &self,
        payer: &Keypair,
        ixs: &[Instruction],
    ) -> eyre::Result<Vec<String>>;
    /// Submit a fully built (already signed) transaction — used when an
    /// instruction needs non-payer signers (e.g. new-account keypairs).
    fn send_transaction(
        &self,
        tx: solana_sdk::transaction::Transaction,
    ) -> eyre::Result<Vec<String>>;
    /// Send a transaction signed by multiple keypairs (evaluation flows
    /// that require two signers, e.g. bootcamp level 2's reveal_secret).
    fn send_signed(
        &self,
        signers: &[&Keypair],
        ixs: &[Instruction],
    ) -> eyre::Result<Vec<String>>;
    fn get_account(&self, addr: &Pubkey) -> Option<Vec<u8>>;
    fn set_account(&self, addr: Pubkey, data: Vec<u8>, owner: Pubkey);
    fn airdrop(&self, addr: &Pubkey, lamports: u64) -> eyre::Result<()>;
    fn latest_blockhash(&self) -> solana_sdk::hash::Hash;
}

/// LiteSVM-backed chain (in-process, no external binaries).
pub struct LiteSvmChain {
    svm: Mutex<litesvm::LiteSVM>,
}

impl LiteSvmChain {
    pub fn new() -> Self {
        Self { svm: Mutex::new(litesvm::LiteSVM::new()) }
    }

    /// A chain that accepts legacy (sbpf v0/v1/v2) programs — required
    /// for older CTF binaries (e.g. Paradigm 2022 hana, built against
    /// solana 1.9) which the default feature set refuses to deploy.
    /// The no-op custom syscall forces litesvm to rebuild its program
    /// runtime environments after the feature-set change.
    pub fn with_legacy_programs() -> Self {
        let mut feature_set = agave_feature_set::FeatureSet::default();
        feature_set.deactivate(
            &agave_feature_set::disable_sbpf_v0_v1_v2_deployment::ID,
        );
        feature_set
            .activate(&agave_feature_set::reenable_sbpf_v0_execution::ID, 0);
        let svm = litesvm::LiteSVM::new()
            .with_feature_set(feature_set)
            .with_custom_syscall("__rebuild_envs__", |_: &mut _, _: &str| {
                Ok(())
            });
        Self { svm: Mutex::new(svm) }
    }
}

impl Default for LiteSvmChain {
    fn default() -> Self {
        Self::new()
    }
}

impl SolChain for LiteSvmChain {
    fn deploy_program(&self, id: Pubkey, elf: &[u8]) -> eyre::Result<()> {
        self.svm
            .lock()
            .map_err(|e| eyre::eyre!("svm lock: {e}"))?
            .add_program(id, elf)
            .map_err(|e| eyre::eyre!("deploy program {id}: {e}"))
    }

    fn deploy_program_with_loader(
        &self,
        id: Pubkey,
        elf: &[u8],
        loader: Pubkey,
    ) -> eyre::Result<()> {
        self.svm
            .lock()
            .map_err(|e| eyre::eyre!("svm lock: {e}"))?
            .add_program_with_loader(id, elf, loader)
            .map_err(|e| eyre::eyre!("deploy program {id}: {e}"))
    }

    fn deploy_program_from_file(
        &self,
        id: Pubkey,
        path: &str,
    ) -> eyre::Result<()> {
        let elf = std::fs::read(path).map_err(|e| {
            eyre::eyre!("Failed to read program path {path}: {e}")
        })?;

        self.deploy_program(id, &elf)
    }

    fn send(
        &self,
        payer: &Keypair,
        ixs: &[Instruction],
    ) -> eyre::Result<Vec<String>> {
        let mut svm =
            self.svm.lock().map_err(|e| eyre::eyre!("svm lock: {e}"))?;
        let msg = Message::new(ixs, Some(&payer.pubkey()));
        let tx = Transaction::new(&[payer], msg, svm.latest_blockhash());
        let result = svm.send_transaction(tx);
        match result {
            Ok(meta) => Ok(meta.logs),
            Err(e) => Err(eyre::eyre!(
                "tx failed: {:?}; meta logs: {:?}",
                e.err,
                e.meta.logs
            )),
        }
    }

    fn send_signed(
        &self,
        signers: &[&Keypair],
        ixs: &[Instruction],
    ) -> eyre::Result<Vec<String>> {
        let msg = Message::new(ixs, Some(&signers[0].pubkey()));
        let blockhash = {
            let svm =
                self.svm.lock().map_err(|e| eyre::eyre!("svm lock: {e}"))?;
            svm.latest_blockhash()
        };
        let tx = Transaction::new(signers, msg, blockhash);
        self.send_transaction(tx)
    }

    fn send_transaction(
        &self,
        tx: solana_sdk::transaction::Transaction,
    ) -> eyre::Result<Vec<String>> {
        let mut svm =
            self.svm.lock().map_err(|e| eyre::eyre!("svm lock: {e}"))?;
        let result = svm.send_transaction(tx);
        match result {
            Ok(meta) => Ok(meta.logs),
            Err(e) => Err(eyre::eyre!(
                "tx failed: {:?}; meta logs: {:?}",
                e.err,
                e.meta.logs
            )),
        }
    }

    fn get_account(&self, addr: &Pubkey) -> Option<Vec<u8>> {
        self.svm.lock().ok()?.get_account(addr).map(|a| a.data)
    }

    fn set_account(&self, addr: Pubkey, data: Vec<u8>, owner: Pubkey) {
        let lamports = 1_000_000_000;
        if let Ok(mut svm) = self.svm.lock() {
            let account = solana_sdk::account::Account {
                lamports,
                data,
                owner,
                executable: false,
                rent_epoch: 0,
            };
            let _ = svm.set_account(addr, account);
        }
    }

    fn airdrop(&self, addr: &Pubkey, lamports: u64) -> eyre::Result<()> {
        self.svm
            .lock()
            .map_err(|e| eyre::eyre!("svm lock: {e}"))?
            .airdrop(addr, lamports)
            .map(|_| ())
            .map_err(|e| eyre::eyre!("airdrop {addr}: {:?}", e))
    }

    fn latest_blockhash(&self) -> solana_sdk::hash::Hash {
        self.svm.lock().map(|svm| svm.latest_blockhash()).unwrap_or_default()
    }
}

/// One actor's signing and account-reading capabilities.
/// The actor does not expose its private key or chain administration.
///
/// ```compile_fail
/// fn no_private_key(actor: &ctf::solana::SolActor) {
///     let _ = &actor.keypair;
/// }
/// ```
///
/// ```compile_fail
/// fn no_chain_administration(actor: &ctf::solana::SolActor) {
///     let _ = &actor.chain;
/// }
/// ```
#[derive(Clone)]
pub struct SolActor {
    keypair: Arc<Keypair>,
    chain: Arc<dyn SolChain>,
}

impl SolActor {
    pub fn pubkey(&self) -> Pubkey {
        self.keypair.pubkey()
    }

    pub fn send(&self, ixs: &[Instruction]) -> eyre::Result<Vec<String>> {
        self.send_signed(&[], ixs)
    }

    /// Read account data without exposing chain administration methods.
    pub fn get_account(&self, addr: &Pubkey) -> Option<Vec<u8>> {
        self.chain.get_account(addr)
    }

    /// Sign with this actor and any additional player-owned keypairs.
    pub fn send_signed(
        &self,
        additional_signers: &[&dyn Signer],
        ixs: &[Instruction],
    ) -> eyre::Result<Vec<String>> {
        let mut signers: Vec<&dyn Signer> = vec![self];
        signers.extend_from_slice(additional_signers);
        let message = Message::new(ixs, Some(&self.pubkey()));
        let mut tx = Transaction::new_unsigned(message);
        tx.try_sign(signers.as_slice(), self.chain.latest_blockhash())?;
        self.chain.send_transaction(tx)
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

/// Three roles mirroring the EVM harness.
pub struct SolRoles {
    pub chain: Arc<dyn SolChain>,
    pub deployer: SolActor,
    pub some_user: SolActor,
    pub offender: SolActor,
}

impl SolRoles {
    pub fn new(chain: Arc<dyn SolChain>) -> Self {
        let deployer = Arc::new(Keypair::new());
        let some_user = Arc::new(Keypair::new());
        let offender = Arc::new(Keypair::new());

        // Fund each with 1000 SOL
        for kp in [&deployer, &some_user, &offender] {
            let _ = chain.airdrop(&kp.pubkey(), 1_000_000_000_000);
        }

        Self {
            chain: chain.clone(),
            deployer: SolActor { keypair: deployer, chain: chain.clone() },
            some_user: SolActor { keypair: some_user, chain: chain.clone() },
            offender: SolActor { keypair: offender, chain },
        }
    }
}

// Re-export the Level and Exploit traits — they are chain-agnostic
// (they use the generic Roles/Actor types from the caller).
pub use crate::level::{
    Exploit as SolExploit, Level as SolLevel, check_exploit,
};

#[cfg(test)]
mod role_boundary_tests {
    use super::*;

    #[test]
    fn actor_signs_only_its_own_identity() -> eyre::Result<()> {
        let chain = Arc::new(LiteSvmChain::new());
        let roles = SolRoles::new(chain.clone());
        let deployer = roles.deployer.pubkey();
        let before = chain.svm.lock().unwrap().get_account(&deployer).unwrap();
        let transfer = solana_system_interface::instruction::transfer(
            &deployer,
            &roles.offender.pubkey(),
            1_000_000,
        );
        assert!(roles.offender.send(&[transfer]).is_err());
        let after = chain.svm.lock().unwrap().get_account(&deployer).unwrap();
        assert_eq!(before.lamports, after.lamports);

        let message = b"actor identity";
        let signature = roles.offender.try_sign_message(message)?;
        assert!(signature.verify(roles.offender.pubkey().as_ref(), message));
        assert!(!signature.verify(deployer.as_ref(), message));
        Ok(())
    }

    #[test]
    fn actor_can_create_accounts_with_player_owned_cosigners()
    -> eyre::Result<()> {
        let chain = Arc::new(LiteSvmChain::new());
        let roles = SolRoles::new(chain.clone());
        let account = Keypair::new();
        let create = solana_system_interface::instruction::create_account(
            &roles.offender.pubkey(),
            &account.pubkey(),
            10_000_000,
            0,
            &Pubkey::default(),
        );
        roles.offender.send_signed(&[&account], &[create])?;
        let created =
            chain.svm.lock().unwrap().get_account(&account.pubkey()).unwrap();
        assert_eq!(created.lamports, 10_000_000);
        assert_eq!(created.owner, Pubkey::default());
        assert_eq!(
            roles.offender.get_account(&account.pubkey()),
            Some(Vec::new())
        );
        Ok(())
    }

    #[test]
    fn player_actor_handles_can_cosign_without_exposing_keys()
    -> eyre::Result<()> {
        let chain = Arc::new(LiteSvmChain::new());
        let roles = SolRoles::new(chain.clone());
        let recipient = Pubkey::new_unique();
        let transfer = solana_system_interface::instruction::transfer(
            &roles.some_user.pubkey(),
            &recipient,
            10_000_000,
        );
        roles.offender.send_signed(&[&roles.some_user], &[transfer])?;
        let received =
            chain.svm.lock().unwrap().get_account(&recipient).unwrap();
        assert_eq!(received.lamports, 10_000_000);
        Ok(())
    }
}
