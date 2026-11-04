//! Ethernaut 35 — Elliptic Token.
//!
//! Goal: Remove all of ALICE's `EllipticToken` tokens after her initial voucher
//! redemption. Her final token balance must be zero.
//!
//
// Upstream hint:
//
// BOB created and owns a new ERC20 token with an elliptic curve–based
// signed voucher redemption system called EllipticToken ($ETK). Bob can
// create vouchers off-chain that can be redeemed on-chain for $ETK. The
// contract also includes a permit system based on elliptic curve signatures.
//
// Bob is a lazy developer and “optimized” some steps of the ECDSA
// algorithm. Can you find the flaw?
//
// Your goal is to steal the $ETK tokens that ALICE
// (`0xA11CE84AcB91Ac59B0A4E2945C9157eF3Ab17D4e`) just redeemed.
//
// Things that might help:
// * Look for any missing step in the [Elliptic Curve Digital Signature
// Algorithm
// (ECDSA)](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm).
//
// Good luck. Elliptic curves do not forgive domain confusion.
//!
//! https://github.com/OpenZeppelin/ethernaut/blob/d05643a/contracts/src/levels/EllipticToken.sol

use async_trait::async_trait;
use ctf::ethereum::ethernaut::lvl35_elliptic_token::*;

pub struct Exploit;

#[async_trait]
impl ctf::Exploit for Exploit {
    type Target = Target;

    async fn attack(
        self,
        target: &Self::Target,
        offender: &ctf::Actor,
    ) -> eyre::Result<()> {
        // TODO: exploit the level.
        let _ = (target, offender);
        Ok(())
    }
}
