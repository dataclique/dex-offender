pub mod hack01_fallback;
pub mod hack02_fallout;
pub mod hack03_coin_flip;
pub mod hack04_telephone;
pub mod hack05_token;
pub mod hack06_delegate;
pub mod hack07_force;
pub mod hack08_vault;
pub mod hack09_king;
pub mod hack10_reentrancy;
pub mod hack11_elevator;
pub mod hack12_privacy;
pub mod hack13_gatekeeper_one;
pub mod hack14_gatekeeper_two;
pub mod hack15_naught_coin;
pub mod hack16_preservation;
pub mod hack17_recovery;
pub mod hack18_magic_number;
pub mod hack19_alien_codex;
pub mod hack20_denial;
pub mod hack21_shop;
pub mod hack22_dex;
pub mod hack23_dex_two;
pub mod hack24_puzzle_wallet;
pub mod hack25_motorbike;
pub mod hack26_double_entry_point;
pub mod hack27_good_samaritan;
pub mod hack28_gatekeeper_three;
pub mod hack29_switch;
// pub mod hack30_higher_order;
// pub mod hack31_stake;
// pub mod hack32_impersonator;
// pub mod hack33_magic_animal_carousel;
// pub mod hack34_bet_house;
// pub mod hack35_elliptic_token;
// pub mod hack36_cashback;
// pub mod hack37_impersonator_two;
// pub mod hack38_unique_nft;
// pub mod hack39_forger;
// pub mod hack40_not_optimistic_portal;

// NOTE: hack01 is the ONLY example solution. All other hacks are intentionally
// left as TODO stubs — the point of this repo is to solve the challenges
// yourself in Rust. Do NOT add exploit implementations to these files.
// hack30–hack40 are commented out until their ctf level modules are wired up.

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() -> eyre::Result<()> {
        println!("Initializing accounts...");
        let roles = ctf::Roles::new(ctf::ANVIL_HTTP)?;

        // Only the example solution runs as a test. To test your own solution,
        // add a line like: ctf::check_exploit(&roles, hackNN::Exploit).await?;
        ctf::check_exploit(&roles, hack01_fallback::Exploit).await?;

        Ok(())
    }
}
