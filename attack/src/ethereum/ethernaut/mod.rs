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
    async fn fallback() -> eyre::Result<()> {
        run(hack01_fallback::Exploit).await
    }

    #[tokio::test]
    async fn fallout() -> eyre::Result<()> {
        run(hack02_fallout::Exploit).await
    }

    #[tokio::test]
    async fn coin_flip() -> eyre::Result<()> {
        run(hack03_coin_flip::Exploit).await
    }

    #[tokio::test]
    async fn telephone() -> eyre::Result<()> {
        run(hack04_telephone::Exploit).await
    }

    #[tokio::test]
    async fn token() -> eyre::Result<()> {
        run(hack05_token::Exploit).await
    }

    #[tokio::test]
    async fn delegate() -> eyre::Result<()> {
        run(hack06_delegate::Exploit).await
    }

    #[tokio::test]
    async fn force() -> eyre::Result<()> {
        run(hack07_force::Exploit).await
    }

    #[tokio::test]
    async fn vault() -> eyre::Result<()> {
        run(hack08_vault::Exploit).await
    }

    #[tokio::test]
    async fn king() -> eyre::Result<()> {
        run(hack09_king::Exploit).await
    }

    #[tokio::test]
    async fn reentrancy() -> eyre::Result<()> {
        run(hack10_reentrancy::Exploit).await
    }

    #[tokio::test]
    async fn elevator() -> eyre::Result<()> {
        run(hack11_elevator::Exploit).await
    }

    #[tokio::test]
    async fn privacy() -> eyre::Result<()> {
        run(hack12_privacy::Exploit).await
    }

    #[tokio::test]
    async fn gatekeeper_one() -> eyre::Result<()> {
        run(hack13_gatekeeper_one::Exploit).await
    }

    #[tokio::test]
    async fn gatekeeper_two() -> eyre::Result<()> {
        run(hack14_gatekeeper_two::Exploit).await
    }

    #[tokio::test]
    async fn naught_coin() -> eyre::Result<()> {
        run(hack15_naught_coin::Exploit).await
    }

    #[tokio::test]
    async fn preservation() -> eyre::Result<()> {
        run(hack16_preservation::Exploit).await
    }

    #[tokio::test]
    async fn recovery() -> eyre::Result<()> {
        run(hack17_recovery::Exploit).await
    }

    #[tokio::test]
    async fn magic_number() -> eyre::Result<()> {
        run(hack18_magic_number::Exploit).await
    }

    #[tokio::test]
    async fn alien_codex() -> eyre::Result<()> {
        run(hack19_alien_codex::Exploit).await
    }

    #[tokio::test]
    async fn denial() -> eyre::Result<()> {
        run(hack20_denial::Exploit).await
    }

    #[tokio::test]
    async fn shop() -> eyre::Result<()> {
        run(hack21_shop::Exploit).await
    }

    #[tokio::test]
    async fn dex() -> eyre::Result<()> {
        run(hack22_dex::Exploit).await
    }

    #[tokio::test]
    async fn dex_two() -> eyre::Result<()> {
        run(hack23_dex_two::Exploit).await
    }

    #[tokio::test]
    async fn puzzle_wallet() -> eyre::Result<()> {
        run(hack24_puzzle_wallet::Exploit).await
    }

    #[tokio::test]
    async fn motorbike() -> eyre::Result<()> {
        run(hack25_motorbike::Exploit).await
    }

    #[tokio::test]
    async fn double_entry_point() -> eyre::Result<()> {
        run(hack26_double_entry_point::Exploit).await
    }

    #[tokio::test]
    async fn good_samaritan() -> eyre::Result<()> {
        run(hack27_good_samaritan::Exploit).await
    }

    #[tokio::test]
    async fn gatekeeper_three() -> eyre::Result<()> {
        run(hack28_gatekeeper_three::Exploit).await
    }

    #[tokio::test]
    async fn switch() -> eyre::Result<()> {
        run(hack29_switch::Exploit).await
    }

    #[tokio::test]
    async fn higher_order() -> eyre::Result<()> {
        run(hack30_higher_order::Exploit).await
    }

    #[tokio::test]
    async fn stake() -> eyre::Result<()> {
        run(hack31_stake::Exploit).await
    }

    #[tokio::test]
    async fn impersonator() -> eyre::Result<()> {
        run(hack32_impersonator::Exploit).await
    }

    #[tokio::test]
    async fn magic_animal_carousel() -> eyre::Result<()> {
        run(hack33_magic_animal_carousel::Exploit).await
    }

    #[tokio::test]
    async fn bet_house() -> eyre::Result<()> {
        run(hack34_bet_house::Exploit).await
    }

    #[tokio::test]
    async fn elliptic_token() -> eyre::Result<()> {
        run(hack35_elliptic_token::Exploit).await
    }

    #[tokio::test]
    async fn cashback() -> eyre::Result<()> {
        run(hack36_cashback::Exploit).await
    }

    #[tokio::test]
    async fn impersonator_two() -> eyre::Result<()> {
        run(hack37_impersonator_two::Exploit).await
    }

    #[tokio::test]
    async fn unique_nft() -> eyre::Result<()> {
        run(hack38_unique_nft::Exploit).await
    }

    #[tokio::test]
    async fn forger() -> eyre::Result<()> {
        run(hack39_forger::Exploit).await
    }

    #[tokio::test]
    async fn not_optimistic_portal() -> eyre::Result<()> {
        run(hack40_not_optimistic_portal::Exploit).await
    }
}
