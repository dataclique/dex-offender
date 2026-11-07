//! Damn Vulnerable DeFi — The Rewarder.
//!
//! Claim all rewards from the distributor using crafted merkle proofs.

use crate::{Level, Roles};
use alloy::{
    network::TransactionBuilder,
    primitives::{Address, B256, U256, keccak256},
    providers::Provider,
    rpc::types::TransactionRequest,
    sol,
    sol_types::SolCall,
};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    DamnValuableToken,
    "../lib/damn-vulnerable-defi/out/DamnValuableToken.sol/DamnValuableToken.json"
);
sol!(
    #[sol(rpc)]
    WETH,
    "../lib/damn-vulnerable-defi/out/WETH.sol/WETH.json"
);
sol!(
    #[sol(rpc)]
    TheRewarderDistributor,
    "../lib/damn-vulnerable-defi/out/TheRewarderDistributor.sol/TheRewarderDistributor.json"
);

/// Upstream constants (test/the-rewarder/TheRewarder.t.sol).
const TOTAL_DVT_DISTRIBUTION_AMOUNT: U256 =
    U256::from_limbs([10_000_000_000_000_000_000, 0, 0, 0]);
const TOTAL_WETH_DISTRIBUTION_AMOUNT: U256 =
    U256::from_limbs([1_000_000_000_000_000_000, 0, 0, 0]);
const ALICE_DVT_CLAIM_AMOUNT: U256 =
    U256::from_limbs([2_502_024_387_994_809, 0, 0, 0]);
const ALICE_WETH_CLAIM_AMOUNT: U256 =
    U256::from_limbs([228_382_988_128_225, 0, 0, 0]);
/// Solved threshold from the upstream success conditions.
const MAX_DVT_LEFT: U256 = U256::from_limbs([10_000_000_000_000_000, 0, 0, 0]);
const MAX_WETH_LEFT: U256 = U256::from_limbs([1_000_000_000_000_000, 0, 0, 0]);

const DVT_DISTRIBUTION_JSON: &str = include_str!(
    "../../../../lib/damn-vulnerable-defi/test/the-rewarder/dvt-distribution.json"
);
const WETH_DISTRIBUTION_JSON: &str = include_str!(
    "../../../../lib/damn-vulnerable-defi/test/the-rewarder/weth-distribution.json"
);

/// One distribution entry: beneficiary address and claimable amount.
#[derive(Debug, serde::Deserialize)]
struct Reward {
    address: Address,
    amount: U256,
}

fn load_rewards(json: &str) -> eyre::Result<Vec<B256>> {
    let rewards: Vec<Reward> = serde_json::from_str(json)?;
    eyre::ensure!(rewards.len() == 1000, "distribution must have 1000 entries");
    // The distributor hashes abi.encodePacked(beneficiary, amount): the
    // 20-byte address followed by the 32-byte big-endian amount.
    Ok(rewards
        .into_iter()
        .map(|reward| {
            let mut leaf = [0u8; 52];
            leaf[..20].copy_from_slice(reward.address.as_slice());
            leaf[20..].copy_from_slice(&reward.amount.to_be_bytes::<32>());
            keccak256(leaf)
        })
        .collect())
}

/// Murky's sorted-pair hash: keccak256(lesser ‖ greater).
fn hash_pair(left: B256, right: B256) -> B256 {
    let (first, second) =
        if left < right { (left, right) } else { (right, left) };
    let mut buffer = [0u8; 64];
    buffer[..32].copy_from_slice(first.as_slice());
    buffer[32..].copy_from_slice(second.as_slice());
    keccak256(buffer)
}

/// Murky's hashLevel: pairwise hashing; an odd trailing element pairs with
/// zero and is appended after the pair results.
fn hash_level(data: &[B256]) -> Vec<B256> {
    let mut result: Vec<B256> = data
        .chunks(2)
        .filter(|chunk| chunk.len() == 2)
        .map(|chunk| hash_pair(chunk[0], chunk[1]))
        .collect();
    if data.len() % 2 == 1 {
        result.push(hash_pair(*data.last().expect("nonempty"), B256::ZERO));
    }
    result
}

/// Murky's root: repeat hashLevel until a single hash remains.
fn merkle_root(data: &[B256]) -> eyre::Result<B256> {
    eyre::ensure!(data.len() > 1, "murky refuses single-leaf roots");
    let mut level = data.to_vec();
    while level.len() > 1 {
        level = hash_level(&level);
    }
    Ok(level[0])
}

/// Murky's proof for `node`: the sibling hash at each level.
fn merkle_proof(data: &[B256], node: usize) -> eyre::Result<Vec<B256>> {
    eyre::ensure!(data.len() > 1, "murky refuses single-leaf proofs");
    let mut level = data.to_vec();
    let mut node = node;
    let mut proof = Vec::new();
    while level.len() > 1 {
        if node % 2 == 1 {
            proof.push(level[node - 1]);
        } else if node + 1 == level.len() {
            proof.push(B256::ZERO);
        } else {
            proof.push(level[node + 1]);
        }
        node /= 2;
        level = hash_level(&level);
    }
    Ok(proof)
}

/// Root and the index-2 beneficiary (upstream's "Alice") for a distribution.
fn distribution(json: &str) -> eyre::Result<(B256, Address)> {
    let leaves = load_rewards(json)?;
    let alice = match serde_json::from_str::<Vec<Reward>>(json)?.get(2) {
        Some(reward) => reward.address,
        None => eyre::bail!("distribution has no index-2 beneficiary"),
    };
    Ok((merkle_root(&leaves)?, alice))
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
    pub dvt: Address,
    pub weth: Address,
    /// The designated recovery account (`some_user`).
    pub recovery: Address,
}

/// Alice's claim for one token, as calldata plus her address.
struct AliceClaim {
    alice: Address,
    calldata: Vec<u8>,
}

/// Impersonate Alice on the owned Anvil and submit her claim transaction.
async fn impersonated_send(
    roles: &Roles,
    claim: &AliceClaim,
    distributor: Address,
) -> eyre::Result<()> {
    let client = roles.provider().client();
    client.request::<_, ()>("anvil_impersonateAccount", (claim.alice,)).await?;
    // Impersonated senders need gas for the claim transaction; funding it
    // is harness mechanics, not challenge state.
    client
        .request::<_, ()>(
            "anvil_setBalance",
            (claim.alice, U256::from(1_000_000_000_000_000_000u64)),
        )
        .await?;
    let pending = client
        .request::<_, alloy::primitives::TxHash>(
            "eth_sendTransaction",
            (TransactionRequest::default()
                .with_from(claim.alice)
                .with_to(distributor)
                .with_input(alloy::primitives::Bytes::from(
                    claim.calldata.clone(),
                )),),
        )
        .await;
    client
        .request::<_, ()>("anvil_stopImpersonatingAccount", (claim.alice,))
        .await?;
    let hash = pending?;
    for _ in 0..100 {
        if let Some(receipt) =
            roles.provider().get_transaction_receipt(hash).await?
        {
            eyre::ensure!(
                receipt.status(),
                "Alice's claim transaction reverted"
            );
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    eyre::bail!("Alice's claim transaction never mined")
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "The Rewarder"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        Ok(ctfs.ethereum.damn_vulnerable_defi.the_rewarder.clone())
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let provider = roles.provider();

        println!("Deploying the The Rewarder challenge...");
        let dvt = DamnValuableToken::deploy(provider).await?;
        let weth = WETH::deploy(provider).await?;
        weth.deposit()
            .value(TOTAL_WETH_DISTRIBUTION_AMOUNT)
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        let (dvt_root, dvt_alice) = distribution(DVT_DISTRIBUTION_JSON)?;
        let (weth_root, weth_alice) = distribution(WETH_DISTRIBUTION_JSON)?;
        eyre::ensure!(
            dvt_alice == weth_alice,
            "Alice differs across distributions"
        );

        let distributor = TheRewarderDistributor::deploy(provider).await?;
        dvt.approve(*distributor.address(), TOTAL_DVT_DISTRIBUTION_AMOUNT)
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        distributor
            .createDistribution(
                *dvt.address(),
                dvt_root,
                TOTAL_DVT_DISTRIBUTION_AMOUNT,
            )
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        weth.approve(*distributor.address(), TOTAL_DVT_DISTRIBUTION_AMOUNT)
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        distributor
            .createDistribution(
                *weth.address(),
                weth_root,
                TOTAL_WETH_DISTRIBUTION_AMOUNT,
            )
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        // Alice (the index-2 beneficiary) claims her share once, like the
        // upstream test setup; the second claim reverts with AlreadyClaimed.
        let dvt_leaves = load_rewards(DVT_DISTRIBUTION_JSON)?;
        let weth_leaves = load_rewards(WETH_DISTRIBUTION_JSON)?;
        let dvt_claim = TheRewarderDistributor::claimRewardsCall {
            inputClaims: vec![TheRewarderDistributor::Claim {
                batchNumber: U256::ZERO,
                amount: ALICE_DVT_CLAIM_AMOUNT,
                tokenIndex: U256::ZERO,
                proof: merkle_proof(&dvt_leaves, 2)?,
            }],
            inputTokens: vec![*dvt.address()],
        };
        let weth_claim = TheRewarderDistributor::claimRewardsCall {
            inputClaims: vec![TheRewarderDistributor::Claim {
                batchNumber: U256::ZERO,
                amount: ALICE_WETH_CLAIM_AMOUNT,
                tokenIndex: U256::ZERO,
                proof: merkle_proof(&weth_leaves, 2)?,
            }],
            inputTokens: vec![*weth.address()],
        };
        impersonated_send(
            roles,
            &AliceClaim { alice: dvt_alice, calldata: dvt_claim.abi_encode() },
            *distributor.address(),
        )
        .await?;
        impersonated_send(
            roles,
            &AliceClaim {
                alice: weth_alice,
                calldata: weth_claim.abi_encode(),
            },
            *distributor.address(),
        )
        .await?;

        // Mirror the upstream initial-state assertions.
        let dvt_contract =
            DamnValuableToken::new(*dvt.address(), provider.clone());
        let weth_contract = WETH::new(*weth.address(), provider.clone());
        eyre::ensure!(
            dvt_contract.balanceOf(dvt_alice).call().await?
                == ALICE_DVT_CLAIM_AMOUNT,
            "Alice's DVT claim amount differs from upstream"
        );
        eyre::ensure!(
            weth_contract.balanceOf(weth_alice).call().await?
                == ALICE_WETH_CLAIM_AMOUNT,
            "Alice's WETH claim amount differs from upstream"
        );
        eyre::ensure!(
            distributor.getNextBatchNumber(*dvt.address()).call().await?
                == U256::from(1),
            "DVT distribution batch missing"
        );
        eyre::ensure!(
            distributor.getNextBatchNumber(*weth.address()).call().await?
                == U256::from(1),
            "WETH distribution batch missing"
        );

        Ok(Target {
            address: *distributor.address(),
            dvt: *dvt.address(),
            weth: *weth.address(),
            recovery: roles.some_user.address(),
        })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let dvt = DamnValuableToken::new(self.dvt, provider.clone());
        let weth = WETH::new(self.weth, provider.clone());
        let distributor_dvt = dvt.balanceOf(self.address).call().await?;
        let distributor_weth = weth.balanceOf(self.address).call().await?;

        println!("Checking that the distributor's DVT and WETH were swept...");
        let swept =
            distributor_dvt < MAX_DVT_LEFT && distributor_weth < MAX_WETH_LEFT;
        let recovered = dvt.balanceOf(self.recovery).call().await?
            == TOTAL_DVT_DISTRIBUTION_AMOUNT
                .saturating_sub(ALICE_DVT_CLAIM_AMOUNT)
                .saturating_sub(distributor_dvt)
            && weth.balanceOf(self.recovery).call().await?
                == TOTAL_WETH_DISTRIBUTION_AMOUNT
                    .saturating_sub(ALICE_WETH_CLAIM_AMOUNT)
                    .saturating_sub(distributor_weth);
        Ok(swept && recovered)
    }
}
