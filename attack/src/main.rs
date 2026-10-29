//! Practice runner — the good part of the loop.
//!
//! `cargo watch -x run` (or the `watch-levels` shell script) reruns this on
//! every save. It plays every registered level against your saved chain:
//! already-solved levels are skipped, freshly solved levels get the
//! CONGRATULATIONS banner, and unsolved stubs land in the end-of-run list.
//! Connects to your practice Anvil at `http://127.0.0.1:8545` (override
//! with `ATTACK_RPC_URL`).

use alloy::providers::Provider;
use attack::ethereum::{damn_vulnerable_defi as dvd, ethernaut, paradigm_2023};
use ctf::{Exploit, Level, Roles};
use std::time::Instant;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";

enum Outcome {
    AlreadySolved,
    NewlySolved,
    Unsolved,
    Skipped(String),
    Failed(String),
}

struct Report {
    name: &'static str,
    outcome: Outcome,
}

async fn attempt<E: Exploit>(
    roles: &Roles,
    label: &'static str,
    exploit: E,
) -> Report {
    let target = match E::Target::from_file() {
        Ok(target) => target,
        Err(error) => {
            return Report {
                name: label,
                outcome: Outcome::Skipped(format!("{error:#}")),
            };
        }
    };
    let name = target.name();
    match target.check(roles).await {
        Ok(true) => Report { name, outcome: Outcome::AlreadySolved },
        Ok(false) => match exploit.attack(&target, &roles.offender).await {
            Err(error) => {
                Report { name, outcome: Outcome::Failed(format!("{error:#}")) }
            }
            Ok(()) => match target.check(roles).await {
                Ok(true) => Report { name, outcome: Outcome::NewlySolved },
                Ok(false) => Report { name, outcome: Outcome::Unsolved },
                Err(error) => Report {
                    name,
                    outcome: Outcome::Failed(format!("{error:#}")),
                },
            },
        },
        Err(error) => {
            Report { name, outcome: Outcome::Skipped(format!("{error:#}")) }
        }
    }
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

fn summary(reports: &[Report], elapsed: std::time::Duration) {
    let solved = reports
        .iter()
        .filter(|r| {
            matches!(r.outcome, Outcome::AlreadySolved | Outcome::NewlySolved)
        })
        .count();
    let unsolved: Vec<&str> = reports
        .iter()
        .filter(|r| matches!(r.outcome, Outcome::Unsolved))
        .map(|r| r.name)
        .collect();

    println!();
    for report in reports {
        match &report.outcome {
            Outcome::AlreadySolved => {
                println!("{DIM}✓ {} — already solved{RESET}", report.name);
            }
            Outcome::NewlySolved => {
                println!("{GREEN}✓ {} — solved just now{RESET}", report.name);
            }
            Outcome::Unsolved => {
                println!("{YELLOW}✗ {} — still unsolved{RESET}", report.name);
            }
            Outcome::Skipped(reason) => {
                println!("{DIM}– {} — skipped: {reason}{RESET}", report.name);
            }
            Outcome::Failed(reason) => {
                println!(
                    "{RED}! {} — attack failed: {reason}{RESET}",
                    report.name
                );
            }
        }
    }

    println!();
    if unsolved.is_empty() && reports.len() == solved {
        println!(
            "{GREEN}{BOLD}All {} levels solved. Legends only.{RESET}",
            reports.len()
        );
    } else {
        println!(
            "{BOLD}Progress: {solved}/{} solved in {:.1}s{RESET}",
            reports.len(),
            elapsed.as_secs_f64()
        );
        if !unsolved.is_empty() {
            println!("{YELLOW}Next up: {}{RESET}", unsolved.join(", "));
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

    let started = Instant::now();
    let mut reports = Vec::new();
    macro_rules! play {
        ($label:literal, $exploit:expr) => {
            reports.push(attempt(&roles, $label, $exploit).await);
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
    play!("Hello World", paradigm_2023::hack_hello_world::Exploit);

    for report in &reports {
        if matches!(report.outcome, Outcome::NewlySolved) {
            congratulations(report.name);
        }
    }
    summary(&reports, started.elapsed());
    Ok(())
}
