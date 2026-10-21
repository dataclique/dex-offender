//! Solana CTF harness core using LiteSVM for in-process program execution.
//!
//! Mirrors the EVM harness in [`crate::roles`] and [`crate::level`]:
//! a [`Roles`] with three actors, a chain abstraction, and the same
//! Level/Exploit traits (re-exported from [`crate::level`]).

use solana_sdk::instruction::Instruction;
use solana_sdk::message::Message;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use std::sync::{Arc, Mutex};

pub use litesvm;

pub mod bootcamp;

/// Abstraction over the Solana execution environment.
/// Default impl wraps `litesvm::LiteSVM` (in-process, no binary needed).
pub trait SolChain: Send + Sync {
    fn deploy_program(&self, id: Pubkey, elf: &[u8]) -> eyre::Result<()>;
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

    fn deploy_program_from_file(
        &self,
        id: Pubkey,
        path: &str,
    ) -> eyre::Result<()> {
        let elf = std::fs::read(path)?;
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
            Err(e) => Err(eyre::eyre!("tx failed: {:?}", e.err)),
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

/// One role's handle: the shared chain plus the role's keypair.
pub struct SolActor {
    pub keypair: Keypair,
    chain: Arc<dyn SolChain>,
}

impl SolActor {
    pub fn pubkey(&self) -> Pubkey {
        self.keypair.pubkey()
    }

    pub fn send(&self, ixs: &[Instruction]) -> eyre::Result<Vec<String>> {
        self.chain.send(&self.keypair, ixs)
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
        let deployer = Keypair::new();
        let some_user = Keypair::new();
        let offender = Keypair::new();

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
