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
pub mod hack30_higher_order;
pub mod hack31_stake;
pub mod hack32_impersonator;
pub mod hack33_magic_animal_carousel;
pub mod hack34_bet_house;
pub mod hack35_elliptic_token;
pub mod hack36_cashback;
pub mod hack37_impersonator_two;
pub mod hack38_unique_nft;
pub mod hack39_forger;
pub mod hack40_not_optimistic_portal;

#[cfg(test)]
mod tests {
    use super::*;

    static LOCAL_CHAIN: tokio::sync::Mutex<()> =
        tokio::sync::Mutex::const_new(());

    async fn run<E: ctf::Exploit>(exploit: E) -> eyre::Result<()> {
        let _guard = LOCAL_CHAIN.lock().await;
        let roles = ctf::Roles::new(ctf::ANVIL_HTTP)?;
        ctf::check_exploit(&roles, exploit).await
    }

    #[tokio::test]
    async fn attack() -> eyre::Result<()> {
        run(hack01_fallback::Exploit).await?;
        run(hack02_fallout::Exploit).await?;
        run(hack03_coin_flip::Exploit).await?;
        run(hack04_telephone::Exploit).await?;
        run(hack05_token::Exploit).await?;
        run(hack06_delegate::Exploit).await?;
        run(hack07_force::Exploit).await?;
        run(hack08_vault::Exploit).await?;
        run(hack09_king::Exploit).await?;
        run(hack10_reentrancy::Exploit).await?;
        run(hack11_elevator::Exploit).await?;
        run(hack12_privacy::Exploit).await?;
        run(hack13_gatekeeper_one::Exploit).await?;
        run(hack14_gatekeeper_two::Exploit).await?;
        run(hack15_naught_coin::Exploit).await?;
        run(hack16_preservation::Exploit).await?;
        run(hack17_recovery::Exploit).await?;
        run(hack18_magic_number::Exploit).await?;
        run(hack19_alien_codex::Exploit).await?;
        run(hack20_denial::Exploit).await?;
        run(hack21_shop::Exploit).await?;
        run(hack22_dex::Exploit).await?;
        run(hack23_dex_two::Exploit).await?;
        run(hack24_puzzle_wallet::Exploit).await?;
        run(hack25_motorbike::Exploit).await?;
        run(hack26_double_entry_point::Exploit).await?;
        run(hack27_good_samaritan::Exploit).await?;
        run(hack28_gatekeeper_three::Exploit).await?;
        run(hack29_switch::Exploit).await?;
        run(hack30_higher_order::Exploit).await?;
        run(hack31_stake::Exploit).await?;
        run(hack32_impersonator::Exploit).await?;
        run(hack33_magic_animal_carousel::Exploit).await?;
        run(hack34_bet_house::Exploit).await?;
        run(hack35_elliptic_token::Exploit).await?;
        run(hack36_cashback::Exploit).await?;
        run(hack37_impersonator_two::Exploit).await?;
        run(hack38_unique_nft::Exploit).await?;
        run(hack39_forger::Exploit).await?;
        run(hack40_not_optimistic_portal::Exploit).await?;
        Ok(())
    }
}
