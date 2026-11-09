//! Paradigm CTF 2022 — hana (Solana), challenge one: fake bitcoin.
//!
//! Goal: Drain the deposit account's token balance to zero, taking the
//! deposited funds from the player's bitcoin + voucher account pair.
//!
//! Sources:
//! https://github.com/paradigmxyz/paradigm-ctf-2022/tree/04a221063cc4c6a43daae6cd305c25f8acc727cb/hana

pub struct Exploit;

pub async fn attack(
    target: &ctf::solana::hana::Target,
    roles: &ctf::solana::SolRoles,
) -> eyre::Result<()> {
    // TODO: exploit the level
    let _ = (target, roles);
    Ok(())
}
