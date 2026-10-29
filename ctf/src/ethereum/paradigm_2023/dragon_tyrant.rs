//! Paradigm CTF 2023 — Dragon Tyrant.
//!
//! Slay the dragon and get the NFT.

use crate::{Level, Roles};
use alloy::{
    network::{ReceiptResponse, TransactionBuilder},
    primitives::{Address, Bytes},
    providers::Provider,
    rpc::types::TransactionRequest,
    sol,
    sol_types::SolValue,
};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    Challenge,
    "abi/paradigm-2023/dragon-tyrant.json"
);

sol!(
    #[sol(rpc)]
    #[sol(ignore_unlinked)]
    Factory,
    "abi/paradigm-2023/Factory.json"
);

sol!(
    #[sol(rpc)]
    NFTRenderer,
    "abi/paradigm-2023/NFTRenderer.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Dragon Tyrant"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs
            .ethereum
            .paradigm_2023
            .dragon_tyrant
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?
            .address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        // Mirror the upstream Deploy script. Factory bytecode links against
        // the NFTRenderer library — deploy it first and patch the
        // placeholder in the creation code.
        let renderer = NFTRenderer::deploy(provider).await?;
        let artifact: serde_json::Value = serde_json::from_str(include_str!(
            "../../../abi/paradigm-2023/Factory.json"
        ))?;
        let unlinked =
            artifact["bytecode"]["object"].as_str().ok_or_else(|| {
                eyre::eyre!("Factory artifact missing bytecode.object")
            })?;
        let lib_hex = format!("{:x}", renderer.address());
        let linked = unlinked
            .replace("__$26dab4df8d9e846ac33e17a73b2e4b2854$__", &lib_hex);
        let code: Bytes = linked.parse()?;
        let receipt = provider
            .send_transaction(
                TransactionRequest::default()
                    .create()
                    .with_from(deployer.address())
                    .with_input(code),
            )
            .await?
            .get_receipt()
            .await?;
        let factory_addr = receipt.contract_address().ok_or_else(|| {
            eyre::eyre!("Factory deploy produced no contract address")
        })?;
        let factory = Factory::new(factory_addr, provider.clone());

        factory
            .setRandomnessOperator(roles.some_user.address())
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        let version = factory.latestItemShopVersion().call().await?;
        let shop_init: alloy::primitives::Bytes =
            ("".to_string(),).abi_encode_params().into();
        let item_shop = factory
            .createItemShop(version, shop_init.clone())
            .from(deployer.address())
            .call()
            .await?;
        factory
            .createItemShop(version, shop_init)
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        let nft_init: alloy::primitives::Bytes =
            ("Fighters".to_string(), "FGHTRS".to_string())
                .abi_encode_params()
                .into();
        let nft_addr = factory
            .createCollection(nft_init.clone())
            .from(deployer.address())
            .call()
            .await?;
        factory
            .createCollection(nft_init)
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        let contract =
            Challenge::deploy(provider, factory_addr, item_shop, nft_addr)
                .await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();

        let contract = Challenge::new(self.address, provider.clone());
        Ok(contract.isSolved().call().await?)
    }
}
