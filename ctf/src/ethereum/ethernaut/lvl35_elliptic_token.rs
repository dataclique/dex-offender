//! Ethernaut 35 — Elliptic Token.
//!
//! Goal: Remove all of ALICE's `EllipticToken` tokens after her initial voucher
//! redemption. Her final token balance must be zero.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/EllipticToken.sol

use crate::{Level, Roles, to_ether};
use alloy::{primitives::Address, primitives::U256, sol};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    EllipticToken,
    "abi/ethernaut/EllipticToken.json"
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "Elliptic Token"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let address = ctfs.ethereum.ethernaut.level35.address;
        Ok(Target { address })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let provider = roles.provider();

        // Mirror EllipticTokenFactory: deploy, transfer ownership to BOB,
        // redeem the known BOB/ALICE voucher to fund ALICE with 10 ether.
        let contract = EllipticToken::deploy(provider).await?;
        let bob = alloy::primitives::address!(
            "B0B14927389CB009E0aabedC271AC29320156Eb8"
        );
        let alice = alloy::primitives::address!(
            "A11CE84AcB91Ac59B0A4E2945C9157eF3Ab17D4e"
        );
        contract
            .transferOwnership(bob)
            .from(roles.deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        let salt = alloy::primitives::keccak256(
            "BOB and ALICE are part of the secret sauce".as_bytes(),
        );
        let bob_sig = alloy::primitives::hex::decode(
            "085a4f70d03930425d3d92b19b9d4e37672a9224ee2cd68381a9854bb3673ef86b35cfdeee0fb1d2168587fb188eefb4fe046109af063bf85d9d3d6859ceb4451c",
        )?
        .into();
        let alice_sig = alloy::primitives::hex::decode(
            "ab1dcd2a2a1c697715a62eb6522b7999d04aa952ffa2619988737ee675d9494f2b50ecce40040bcb29b5a8ca1da875968085f22b7c0a50f29a4851396251de121c",
        )?
        .into();
        contract
            .redeemVoucher(to_ether(10), alice, salt, bob_sig, alice_sig)
            .from(roles.deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        Ok(Target { address: *contract.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let provider = roles.provider();
        let contract = EllipticToken::new(self.address, provider.clone());

        // ALICE = 0xA11CE84AcB91Ac59B0A4E2945C9157eF3Ab17D4e
        let alice = alloy::primitives::address!(
            "A11CE84AcB91Ac59B0A4E2945C9157eF3Ab17D4e"
        );
        let alice_balance = contract.balanceOf(alice).call().await?;
        Ok(alice_balance == U256::ZERO)
    }
}
