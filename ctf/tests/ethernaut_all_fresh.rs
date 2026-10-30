//! Every Ethernaut level must deploy, check without error, and start
//! unsolved on an owned Anvil (never the player's node).

use ctf::{Level, Roles, ethereum::ethernaut};
use std::{
    net::{TcpListener, TcpStream},
    process::{Child, Command, Stdio},
    time::Duration,
};

/// Owned Anvil on an OS-assigned port; killed and reaped on drop.
struct OwnedAnvil {
    child: Child,
    port: u16,
}

impl OwnedAnvil {
    fn spawn() -> std::io::Result<Self> {
        let reservation = TcpListener::bind(("127.0.0.1", 0))?;
        let port = reservation.local_addr()?.port();
        drop(reservation);
        let child = Command::new("anvil")
            .args(["--port", &port.to_string(), "--quiet"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        Ok(Self { child, port })
    }

    fn wait_ready(&mut self) -> std::io::Result<()> {
        for _ in 0..100 {
            if let Some(status) = self.child.try_wait()? {
                return Err(std::io::Error::other(format!(
                    "owned Anvil exited: {status}"
                )));
            }
            if TcpStream::connect(("127.0.0.1", self.port)).is_ok() {
                return Ok(());
            }
            std::thread::sleep(Duration::from_millis(50));
        }
        Err(std::io::Error::other("owned Anvil never became ready"))
    }

    fn stop(&mut self) -> std::io::Result<()> {
        if self.child.try_wait()?.is_none() {
            self.child.kill()?;
        }
        self.child.wait()?;
        Ok(())
    }
}

macro_rules! fresh_check {
    ($test_name:ident, $module:path) => {
        #[tokio::test]
        async fn $test_name() -> eyre::Result<()> {
            let mut anvil = OwnedAnvil::spawn()?;
            let result: eyre::Result<()> = async {
                anvil.wait_ready()?;
                let roles =
                    Roles::new(&format!("http://127.0.0.1:{}", anvil.port))?;
                let level = <$module as Level>::set_up(&roles)
                    .await
                    .map_err(|e| eyre::eyre!("set_up failed: {e:#}"))?;
                let solved = level
                    .check(&roles)
                    .await
                    .map_err(|e| eyre::eyre!("fresh check errored: {e:#}"))?;
                eyre::ensure!(!solved, "fresh deployment reports solved");
                Ok(())
            }
            .await;
            anvil.stop()?;
            result
        }
    };
}

fresh_check!(lvl01_fresh, ethernaut::lvl01_fallback::Target);
fresh_check!(lvl02_fresh, ethernaut::lvl02_fallout::Target);
fresh_check!(lvl03_fresh, ethernaut::lvl03_coin_flip::Target);
fresh_check!(lvl04_fresh, ethernaut::lvl04_telephone::Target);
fresh_check!(lvl05_fresh, ethernaut::lvl05_token::Target);
fresh_check!(lvl06_fresh, ethernaut::lvl06_delegate::Target);
fresh_check!(lvl07_fresh, ethernaut::lvl07_force::Target);
fresh_check!(lvl08_fresh, ethernaut::lvl08_vault::Target);
fresh_check!(lvl09_fresh, ethernaut::lvl09_king::Target);
fresh_check!(lvl10_fresh, ethernaut::lvl10_reentrancy::Target);
fresh_check!(lvl11_fresh, ethernaut::lvl11_elevator::Target);
fresh_check!(lvl12_fresh, ethernaut::lvl12_privacy::Target);
fresh_check!(lvl13_fresh, ethernaut::lvl13_gatekeeper_one::Target);
fresh_check!(lvl14_fresh, ethernaut::lvl14_gatekeeper_two::Target);
fresh_check!(lvl15_fresh, ethernaut::lvl15_naught_coin::Target);
fresh_check!(lvl16_fresh, ethernaut::lvl16_preservation::Target);
fresh_check!(lvl17_fresh, ethernaut::lvl17_recovery::Target);
fresh_check!(lvl18_fresh, ethernaut::lvl18_magic_number::Target);
fresh_check!(lvl19_fresh, ethernaut::lvl19_alien_codex::Target);
fresh_check!(lvl20_fresh, ethernaut::lvl20_denial::Target);
fresh_check!(lvl21_fresh, ethernaut::lvl21_shop::Target);
fresh_check!(lvl22_fresh, ethernaut::lvl22_dex::Target);
fresh_check!(lvl23_fresh, ethernaut::lvl23_dex_two::Target);
fresh_check!(lvl24_fresh, ethernaut::lvl24_puzzle_wallet::Target);
fresh_check!(lvl25_fresh, ethernaut::lvl25_motorbike::Target);
fresh_check!(lvl26_fresh, ethernaut::lvl26_double_entry_point::Target);
fresh_check!(lvl27_fresh, ethernaut::lvl27_good_samaritan::Target);
fresh_check!(lvl28_fresh, ethernaut::lvl28_gatekeeper_three::Target);
fresh_check!(lvl29_fresh, ethernaut::lvl29_switch::Target);
fresh_check!(lvl30_fresh, ethernaut::lvl30_higher_order::Target);
fresh_check!(lvl31_fresh, ethernaut::lvl31_stake::Target);
fresh_check!(lvl32_fresh, ethernaut::lvl32_impersonator::Target);
fresh_check!(lvl33_fresh, ethernaut::lvl33_magic_animal_carousel::Target);
fresh_check!(lvl34_fresh, ethernaut::lvl34_bet_house::Target);
fresh_check!(lvl35_fresh, ethernaut::lvl35_elliptic_token::Target);
fresh_check!(lvl36_fresh, ethernaut::lvl36_cashback::Target);
fresh_check!(lvl37_fresh, ethernaut::lvl37_impersonator_two::Target);
fresh_check!(lvl38_fresh, ethernaut::lvl38_unique_nft::Target);
fresh_check!(lvl39_fresh, ethernaut::lvl39_forger::Target);
fresh_check!(lvl40_fresh, ethernaut::lvl40_not_optimistic_portal::Target);
