//! Role accounts for the local anvil chain (alloy-based).
//!
//! Each actor has a [`DynProvider`] whose wallet defaults to that actor.
//! All three local signers remain registered for explicit `from` overrides;
//! [`Roles::provider`] keeps the deployer default for challenge setup.

use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{Address, U256},
    providers::{DynProvider, Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use std::sync::Arc;

pub const ANVIL_HTTP: &str = "http://127.0.0.1:8545";

pub type ProviderT = DynProvider;

/// One role's handle: its role-default provider and address.
#[derive(Clone)]
pub struct Actor {
    provider: Arc<ProviderT>,
    address: Address,
}

impl Actor {
    pub fn new(provider: Arc<ProviderT>, address: Address) -> Self {
        Self { provider, address }
    }

    pub fn address(&self) -> Address {
        self.address
    }

    pub fn provider(&self) -> &Arc<ProviderT> {
        &self.provider
    }

    /// Send a plain ETH transfer to `to` for `value` wei, signed by this
    /// role's registered signer. Errors if the receipt reports failure.
    pub async fn transfer(&self, to: Address, value: U256) -> eyre::Result<()> {
        let tx = TransactionRequest::default()
            .with_from(self.address)
            .with_to(to)
            .with_value(value);
        let receipt =
            self.provider.send_transaction(tx).await?.get_receipt().await?;
        if !receipt.status() {
            return Err(eyre::eyre!(
                "transfer from {} to {} reverted",
                self.address,
                to
            ));
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct Roles {
    provider: Arc<ProviderT>,
    pub deployer: Actor,
    pub some_user: Actor,
    pub offender: Actor,
}

impl Roles {
    /// The three canonical Anvil accounts (see the README table). These are
    /// the well-known public anvil dev keys.
    pub fn new(url: &str) -> eyre::Result<Self> {
        let deployer_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let some_user_key = "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
        let offender_key = "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6";

        let deployer: PrivateKeySigner = deployer_key.parse()?;
        let some_user: PrivateKeySigner = some_user_key.parse()?;
        let offender: PrivateKeySigner = offender_key.parse()?;

        let mut wallet = EthereumWallet::new(deployer.clone());
        wallet.register_signer(some_user.clone());
        wallet.register_signer(offender.clone());

        let actor = |signer: &PrivateKeySigner| -> eyre::Result<Actor> {
            let mut role_wallet = wallet.clone();
            role_wallet.set_default_signer(signer.address())?;
            // Failed pre-broadcast attempts must not reserve a cached nonce
            // and leave subsequent sequential transactions behind a gap.
            let provider = ProviderBuilder::new()
                .disable_recommended_fillers()
                .with_gas_estimation()
                .with_blob_gas_estimation()
                .with_simple_nonce_management()
                .fetch_chain_id()
                .wallet(role_wallet)
                .connect_http(url.parse()?)
                .erased();
            Ok(Actor::new(Arc::new(provider), signer.address()))
        };

        let deployer = actor(&deployer)?;
        let some_user = actor(&some_user)?;
        let offender = actor(&offender)?;

        Ok(Roles {
            provider: deployer.provider.clone(),
            deployer,
            some_user,
            offender,
        })
    }

    pub fn provider(&self) -> &Arc<ProviderT> {
        &self.provider
    }
}
