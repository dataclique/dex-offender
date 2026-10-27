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
    use ctf::Level;

    static LOCAL_CHAIN: tokio::sync::Mutex<()> =
        tokio::sync::Mutex::const_new(());

    // Each run deploys a fresh instance; no saved board or previous solutions.
    async fn run<E: ctf::Exploit>(exploit: E) -> eyre::Result<()> {
        let _guard = LOCAL_CHAIN.lock().await;
        let roles = ctf::Roles::new(ctf::ANVIL_HTTP)?;
        let target = E::Target::set_up(&roles).await?;

        eyre::ensure!(
            !target.check(&roles).await?,
            "fresh level is already solved"
        );

        exploit.attack(&target, &roles.offender).await?;

        eyre::ensure!(
            target.check(&roles).await?,
            "attack ran but the level is still unsolved"
        );

        println!("Solved {}", target.name());
        Ok(())
    }

    #[tokio::test]
    async fn fallout_starts_unsolved() -> eyre::Result<()> {
        let _guard = LOCAL_CHAIN.lock().await;
        let roles = ctf::Roles::new(ctf::ANVIL_HTTP)?;
        let target =
            ctf::ethereum::ethernaut::lvl02_fallout::Target::set_up(&roles)
                .await?;
        eyre::ensure!(
            !target.check(&roles).await?,
            "fresh Fallout is already solved"
        );
        Ok(())
    }

    #[tokio::test]
    async fn fallback() -> eyre::Result<()> {
        run(hack01_fallback::Exploit).await
    }

    #[tokio::test]
    #[ignore = "Implement the blank hack module, then run this test with --ignored"]
    async fn fallout() -> eyre::Result<()> {
        run(hack02_fallout::Exploit).await
    }

    #[tokio::test]
    #[ignore = "Implement the blank hack module, then run this test with --ignored"]
    async fn coin_flip() -> eyre::Result<()> {
        run(hack03_coin_flip::Exploit).await
    }

    #[tokio::test]
    #[ignore = "Implement the blank hack module, then run this test with --ignored"]
    async fn telephone() -> eyre::Result<()> {
        run(hack04_telephone::Exploit).await
    }

    #[tokio::test]
    #[ignore = "Implement the blank hack module, then run this test with --ignored"]
    async fn token() -> eyre::Result<()> {
        run(hack05_token::Exploit).await
    }

    #[tokio::test]
    #[ignore = "Implement the blank hack module, then run this test with --ignored"]
    async fn delegate() -> eyre::Result<()> {
        run(hack06_delegate::Exploit).await
    }

    #[tokio::test]
    #[ignore = "Implement the blank hack module, then run this test with --ignored"]
    async fn force() -> eyre::Result<()> {
        run(hack07_force::Exploit).await
    }

    #[tokio::test]
    #[ignore = "Implement the blank hack module, then run this test with --ignored"]
    async fn vault() -> eyre::Result<()> {
        run(hack08_vault::Exploit).await
    }

    #[tokio::test]
    #[ignore = "Implement the blank hack module, then run this test with --ignored"]
    async fn king() -> eyre::Result<()> {
        run(hack09_king::Exploit).await
    }

    #[tokio::test]
    #[ignore = "Implement the blank hack module, then run this test with --ignored"]
    async fn reentrancy() -> eyre::Result<()> {
        run(hack10_reentrancy::Exploit).await
    }
}
