//! Practice runner — the good part of the loop.
//!
//! `cargo watch -x run` (or the `watch-levels` shell script) reruns this on
//! every save. Levels play in a fixed order — Ethernaut 1–40, then Paradigm
//! 2023, then Damn Vulnerable DeFi (strictly harder) — and the run stops at
//! the first level that is not solved: earlier levels are re-checked, a
//! level you just solved gets the CONGRATULATIONS banner, and nothing after
//! your current level is touched. Connects to your practice Anvil at
//! `http://127.0.0.1:8545` (override with `ATTACK_RPC_URL`).

use alloy::providers::Provider;
use attack::ethereum::{damn_vulnerable_defi as dvd, ethernaut, paradigm_2023};
use ctf::{Exploit, Level, Roles};

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";

const TOTAL: usize = 58;

enum Outcome {
    AlreadySolved,
    NewlySolved,
    Unsolved,
    Uncheckable(String),
    Failed(String),
}

async fn attempt<E: Exploit>(
    roles: &Roles,
    label: &'static str,
    exploit: E,
) -> Outcome {
    let target = match E::Target::from_file() {
        Ok(target) => target,
        Err(error) => {
            return Outcome::Uncheckable(format!("{error:#}"));
        }
    };
    let _ = label;
    match target.check(roles).await {
        Ok(true) => Outcome::AlreadySolved,
        Ok(false) => match exploit.attack(&target, &roles.offender).await {
            Err(error) => Outcome::Failed(format!("{error:#}")),
            Ok(()) => match target.check(roles).await {
                Ok(true) => Outcome::NewlySolved,
                Ok(false) => Outcome::Unsolved,
                Err(error) => Outcome::Failed(format!("{error:#}")),
            },
        },
        Err(error) => Outcome::Uncheckable(format!("{error:#}")),
    }
}

/// First line only: full RPC error chains are noise in the status list.
fn first_line(message: &str) -> &str {
    message.lines().next().unwrap_or("unknown error")
}

/// The chain must actually contain the contracts the saved board points
/// at. A mismatch (fresh deploy, stale anvil state) would otherwise turn
/// every level into a misleading `cannot check` line.
async fn verify_board_matches_chain(roles: &Roles) -> eyre::Result<()> {
    let board = ctf::CTFs::from_file()?;
    let probe = board.ethereum.ethernaut.level01.address;
    let code = roles.provider().get_code_at(probe).await?;
    eyre::ensure!(
        !code.is_empty(),
        "your chain has no contract at the board's level-01 address ({probe})\n\
         The running Anvil does not match ctfs.json. Restart it from the\n\
         matching state (mind your working directory):\n\
         anvil --state state.json --code-size-limit 50000\n\
         (redeploy first with: cargo run -p ctf --bin deploy_levels)"
    );
    Ok(())
}

fn congratulations(name: &str) {
    println!(
        "
{GREEN}{BOLD}-------------------------------------
//////     CONGRATULATIONS     //////

$$$  Y O U  H A V E  S O L V E D  $$$
$$$   T H E   C H A L L E N G E   $$$
        {name}

youpassedthelevelyoupassedthelevelyou
passedthelevelyoupassedthelevelyoupas
sedthelevelyoupassedthelevelyoupassed
-------------------------------------{RESET}"
    );
}

/// Play one level in order. Returns false when the run stops here.
async fn play_level<E: Exploit>(
    roles: &Roles,
    label: &'static str,
    exploit: E,
    solved: &mut usize,
) -> bool {
    match attempt(roles, label, exploit).await {
        Outcome::AlreadySolved => {
            println!("{DIM}✓ {label} — already solved{RESET}");
            *solved += 1;
            true
        }
        Outcome::NewlySolved => {
            congratulations(label);
            println!("{GREEN}✓ {label} — solved just now{RESET}");
            *solved += 1;
            true
        }
        Outcome::Unsolved => {
            println!("{YELLOW}✗ {label} — your next level{RESET}");
            println!("{YELLOW}Next up: {label}{RESET}");
            println!("{BOLD}Progress: {solved}/{TOTAL} solved{RESET}");
            false
        }
        Outcome::Uncheckable(reason) => {
            println!(
                "{RED}! {label} — cannot check: {}{RESET}",
                first_line(&reason)
            );
            println!("{YELLOW}Blocked at: {label}{RESET}");
            println!("{BOLD}Progress: {solved}/{TOTAL} solved{RESET}");
            false
        }
        Outcome::Failed(reason) => {
            println!(
                "{RED}! {label} — attack failed: {}{RESET}",
                first_line(&reason)
            );
            println!("{YELLOW}Next up: {label}{RESET}");
            println!("{BOLD}Progress: {solved}/{TOTAL} solved{RESET}");
            false
        }
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let url = std::env::var("ATTACK_RPC_URL")
        .unwrap_or_else(|_| ctf::ANVIL_HTTP.to_string());
    let roles = Roles::new(&url)?;
    if let Err(error) = roles.provider().get_block_number().await {
        eprintln!("{RED}Cannot reach your practice chain at {url}.{RESET}");
        eprintln!(
            "Start it with: anvil --state state.json --code-size-limit 50000"
        );
        eprintln!(
            "No saved deployment yet? Run: cargo run -p ctf --bin deploy_levels"
        );
        return Err(error.into());
    }
    verify_board_matches_chain(&roles).await?;

    // Ordered progression: Ethernaut 1-40, Paradigm 2023, then the
    // strictly harder Damn Vulnerable DeFi suite last. The run stops at
    // the first level that is not solved — nothing later is checked.
    let mut solved = 0usize;
    macro_rules! play {
        ($label:literal, $exploit:expr) => {
            if !play_level(&roles, $label, $exploit, &mut solved).await {
                return Ok(());
            }
        };
    }

    play!("Fallback", ethernaut::hack01_fallback::Exploit);
    play!("Fallout", ethernaut::hack02_fallout::Exploit);
    play!("Coin Flip", ethernaut::hack03_coin_flip::Exploit);
    play!("Telephone", ethernaut::hack04_telephone::Exploit);
    play!("Token", ethernaut::hack05_token::Exploit);
    play!("Delegation", ethernaut::hack06_delegate::Exploit);
    play!("Force", ethernaut::hack07_force::Exploit);
    play!("Vault", ethernaut::hack08_vault::Exploit);
    play!("King", ethernaut::hack09_king::Exploit);
    play!("Re-entrancy", ethernaut::hack10_reentrancy::Exploit);
    play!("Elevator", ethernaut::hack11_elevator::Exploit);
    play!("Privacy", ethernaut::hack12_privacy::Exploit);
    play!("GatekeeperOne", ethernaut::hack13_gatekeeper_one::Exploit);
    play!("GatekeeperTwo", ethernaut::hack14_gatekeeper_two::Exploit);
    play!("NaughtCoin", ethernaut::hack15_naught_coin::Exploit);
    play!("Preservation", ethernaut::hack16_preservation::Exploit);
    play!("Recovery", ethernaut::hack17_recovery::Exploit);
    play!("MagicNum", ethernaut::hack18_magic_number::Exploit);
    play!("AlienCodex", ethernaut::hack19_alien_codex::Exploit);
    play!("Denial", ethernaut::hack20_denial::Exploit);
    play!("Shop", ethernaut::hack21_shop::Exploit);
    play!("Dex", ethernaut::hack22_dex::Exploit);
    play!("DexTwo", ethernaut::hack23_dex_two::Exploit);
    play!("PuzzleWallet", ethernaut::hack24_puzzle_wallet::Exploit);
    play!("Motorbike", ethernaut::hack25_motorbike::Exploit);
    play!("DoubleEntryPoint", ethernaut::hack26_double_entry_point::Exploit);
    play!("GoodSamaritan", ethernaut::hack27_good_samaritan::Exploit);
    play!("GatekeeperThree", ethernaut::hack28_gatekeeper_three::Exploit);
    play!("Switch", ethernaut::hack29_switch::Exploit);
    play!("HigherOrder", ethernaut::hack30_higher_order::Exploit);
    play!("Stake", ethernaut::hack31_stake::Exploit);
    play!("Impersonator", ethernaut::hack32_impersonator::Exploit);
    play!(
        "Magic Animal Carousel",
        ethernaut::hack33_magic_animal_carousel::Exploit
    );
    play!("Bet House", ethernaut::hack34_bet_house::Exploit);
    play!("Elliptic Token", ethernaut::hack35_elliptic_token::Exploit);
    play!("Cashback", ethernaut::hack36_cashback::Exploit);
    play!("Impersonator Two", ethernaut::hack37_impersonator_two::Exploit);
    play!("UniqueNFT", ethernaut::hack38_unique_nft::Exploit);
    play!("Forger", ethernaut::hack39_forger::Exploit);
    play!(
        "NotOptimisticPortal",
        ethernaut::hack40_not_optimistic_portal::Exploit
    );
    play!("Hello World", paradigm_2023::hack_hello_world::Exploit);
    play!("Unstoppable", dvd::hack01_unstoppable::Exploit);
    play!("Naive Receiver", dvd::hack02_naive_receiver::Exploit);
    play!("Truster", dvd::hack03_truster::Exploit);
    play!("Side Entrance", dvd::hack04_side_entrance::Exploit);
    play!("The Rewarder", dvd::hack05_the_rewarder::Exploit);
    play!("Selfie", dvd::hack06_selfie::Exploit);
    play!("Compromised", dvd::hack07_compromised::Exploit);
    play!("Puppet", dvd::hack08_puppet::Exploit);
    play!("PuppetV2", dvd::hack09_puppet_v2::Exploit);
    play!("FreeRider", dvd::hack10_free_rider::Exploit);
    play!("Backdoor", dvd::hack11_backdoor::Exploit);
    play!("Climber", dvd::hack12_climber::Exploit);
    play!("Wallet Mining", dvd::hack13_wallet_mining::Exploit);
    play!("PuppetV3", dvd::hack14_puppet_v3::Exploit);
    play!("ABI Smuggling", dvd::hack15_abi_smuggling::Exploit);
    play!("Shards", dvd::hack16_shards::Exploit);
    play!("Withdrawal", dvd::hack17_withdrawal::Exploit);

    println!("{GREEN}{BOLD}All {TOTAL} levels solved. Legends only.{RESET}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        net::{TcpListener, TcpStream},
        process::{Child, Command, Stdio},
        time::Duration,
    };

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

    #[tokio::test]
    async fn stale_board_against_empty_chain_is_rejected() {
        let mut anvil = OwnedAnvil::spawn().expect("spawn owned anvil");
        let result: eyre::Result<()> = async {
            anvil.wait_ready()?;
            let roles =
                Roles::new(&format!("http://127.0.0.1:{}", anvil.port))?;
            verify_board_matches_chain(&roles).await
        }
        .await;
        anvil.stop().expect("stop owned anvil");
        assert!(
            result.is_err(),
            "board addresses without code on-chain must be rejected"
        );
    }
}
