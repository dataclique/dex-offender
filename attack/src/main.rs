//! Practice runner — the good part of the loop.
//!
//! `cargo watch -x run` (or the `watch-levels` shell script) reruns this on
//! every save. Flip `TRACK` below to switch between the Ethereum and
//! Solana tracks. Levels play in a fixed order — Ethernaut 1–40, then
//! Paradigm 2023, then Damn Vulnerable DeFi (strictly harder) on Ethereum;
//! Ackee 1–5 on Solana — and the run stops
//! at the first level that is not solved: earlier levels are re-checked, a
//! level you just solved gets the CONGRATULATIONS banner, and nothing after
//! your current level is touched. Connects to your practice Anvil at
//! `http://127.0.0.1:8545` (override with `ATTACK_RPC_URL`) on Ethereum;
//! Solana connects to the persistent local Surfpool RPC endpoint.

use alloy::providers::Provider;
use attack::ethereum::{damn_vulnerable_defi as dvd, ethernaut, paradigm_2023};
use attack::hints;
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

/// Upstream "completion" notes shown right after a level is newly solved
/// (owner-approved: not a spoiler once the level is completed). Only
/// Ethernaut ships these; Bet House has none upstream.
const ETERNAUT_LEARNINGS: &[(&str, &str)] = &[
    (
        "Fallback",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/fallback_complete.md"
        ),
    ),
    (
        "Fallout",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/fallout_complete.md"
        ),
    ),
    (
        "Coin Flip",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/coinflip_complete.md"
        ),
    ),
    (
        "Telephone",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/telephone_complete.md"
        ),
    ),
    (
        "Token",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/token_complete.md"
        ),
    ),
    (
        "Delegation",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/delegate_complete.md"
        ),
    ),
    (
        "Force",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/force_complete.md"
        ),
    ),
    (
        "Vault",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/vault_complete.md"
        ),
    ),
    (
        "King",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/king_complete.md"
        ),
    ),
    (
        "Re-entrancy",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/reentrancy_complete.md"
        ),
    ),
    (
        "Elevator",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/elevator_complete.md"
        ),
    ),
    (
        "Privacy",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/privacy_complete.md"
        ),
    ),
    (
        "GatekeeperOne",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/gatekeeper1_complete.md"
        ),
    ),
    (
        "GatekeeperTwo",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/gatekeeper2_complete.md"
        ),
    ),
    (
        "NaughtCoin",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/naughtcoin_complete.md"
        ),
    ),
    (
        "Preservation",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/preservation_complete.md"
        ),
    ),
    (
        "Recovery",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/recovery_complete.md"
        ),
    ),
    (
        "MagicNum",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/magicnum_complete.md"
        ),
    ),
    (
        "AlienCodex",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/aliencodex_complete.md"
        ),
    ),
    (
        "Denial",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/denial_complete.md"
        ),
    ),
    (
        "Shop",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/shop_complete.md"
        ),
    ),
    (
        "Dex",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/dex_complete.md"
        ),
    ),
    (
        "DexTwo",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/dex2_complete.md"
        ),
    ),
    (
        "PuzzleWallet",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/puzzle_wallet_complete.md"
        ),
    ),
    (
        "Motorbike",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/motorbike_complete.md"
        ),
    ),
    (
        "DoubleEntryPoint",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/doubleentrypoint_complete.md"
        ),
    ),
    (
        "GoodSamaritan",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/goodsamaritan_complete.md"
        ),
    ),
    (
        "GatekeeperThree",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/gatekeeper3_complete.md"
        ),
    ),
    (
        "Switch",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/switch_complete.md"
        ),
    ),
    (
        "HigherOrder",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/higherorder_complete.md"
        ),
    ),
    (
        "Stake",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/stake_complete.md"
        ),
    ),
    (
        "Impersonator",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/impersonator_complete.md"
        ),
    ),
    (
        "Magic Animal Carousel",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/magicanimalcarousel_complete.md"
        ),
    ),
    ("Bet House", ""),
    (
        "Elliptic Token",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/elliptic_token_complete.md"
        ),
    ),
    (
        "Cashback",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/cashback_complete.md"
        ),
    ),
    (
        "Impersonator Two",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/impersonator_two_complete.md"
        ),
    ),
    (
        "UniqueNFT",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/uniquenft_complete.md"
        ),
    ),
    (
        "Forger",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/forger_complete.md"
        ),
    ),
    (
        "NotOptimisticPortal",
        include_str!(
            "../../lib/ethernaut/client/src/gamedata/en/descriptions/levels/not_optimistic_portal_complete.md"
        ),
    ),
];

fn learnings_for(name: &str) -> Option<&'static str> {
    ETERNAUT_LEARNINGS
        .iter()
        .find(|(level, _)| *level == name)
        .map(|(_, text)| *text)
        .filter(|text| !text.is_empty())
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
            if let Some(learnings) = learnings_for(label) {
                println!(
                    "{DIM}What you learned:{RESET}\n{}",
                    hints::render_markdown(learnings)
                );
            }
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

/// Which wargame track the runner plays. Flip this const to switch
/// tracks — everything else (watch-levels, banners, progression gate)
/// works the same on both.
#[allow(dead_code)]
enum Track {
    Ethereum,
    Solana,
}

const TRACK: Track = Track::Solana;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    match TRACK {
        Track::Ethereum => ethereum_track().await,
        Track::Solana => solana_track().await,
    }
}

/// Ackee Bootcamp levels 1–5 on the persistent Surfpool chain, played
/// in order with the same outcome semantics as the Ethereum track.
async fn solana_track() -> eyre::Result<()> {
    use attack::solana::ackee::{
        hack1_ancient_vault, hack2_monster_battle, hack3_hidden_faction,
        hack4_vesting, hack5_guardian_escrow,
    };
    use ctf::solana::ackee::{
        self, level_1, level_2, level_3, level_4, level_5,
    };
    use ctf::solana::{SolActor, rpc};
    use eyre::WrapErr as _;

    let offender = SolActor::from_file(&rpc::rpc_url()?, rpc::keypair_path())?;
    let actors = ackee::AckeeActors::load_or_create(&rpc::ackee_actors_path())
        .wrap_err_with(|| {
            "cannot load Ackee setup identities; \
             run cargo run -p ctf --bin deploy_levels -- solana"
        })?;
    let rpc = offender.rpc_client();
    let hacker = offender.keypair.as_ref();
    let mut solved = 0usize;

    macro_rules! play {
        ($label:literal, $load:expr, $check:expr, $attack:expr, $hint:expr) => {{
            let label: &str = $label;
            let hint: Option<&str> = $hint;
            let outcome = match $load.await {
                Err(error) => Outcome::Uncheckable(format!("{error:#}")),
                Ok(target) => {
                    let check = &$check;
                    let attack = $attack;
                    match check(target.clone()).await {
                        Err(error) => {
                            Outcome::Uncheckable(format!("{error:#}"))
                        }
                        Ok(true) => Outcome::AlreadySolved,
                        Ok(false) => match attack(&target, &offender).await {
                            Err(error) => Outcome::Failed(format!("{error:#}")),
                            Ok(()) => match check(target.clone()).await {
                                Err(error) => {
                                    Outcome::Uncheckable(format!("{error:#}"))
                                }
                                Ok(true) => Outcome::NewlySolved,
                                Ok(false) => Outcome::Unsolved,
                            },
                        },
                    }
                }
            };
            match outcome {
                Outcome::AlreadySolved => {
                    println!("{DIM}✓ {label} — already solved{RESET}");
                    solved += 1;
                }
                Outcome::NewlySolved => {
                    congratulations(label);
                    println!("{GREEN}✓ {label} — solved just now{RESET}");
                    solved += 1;
                }
                Outcome::Unsolved => {
                    println!("{YELLOW}✗ {label} — not solved yet{RESET}");
                    if let Some(hint) = hint {
                        println!("{DIM}{hint}{RESET}");
                    }
                    println!("\n{BOLD}Next up:{RESET} {label}");
                    println!(
                        "{DIM}Progress: {solved}/5 enabled levels solved{RESET}"
                    );
                    return Ok(());
                }
                Outcome::Uncheckable(reason) => {
                    println!(
                        "{RED}! {label} — cannot check: {}{RESET}",
                        first_line(&reason)
                    );
                    println!("{YELLOW}Blocked at: {label}{RESET}");
                    println!("{BOLD}Progress: {solved}/5 solved{RESET}");
                    return Ok(());
                }
                Outcome::Failed(reason) => {
                    println!(
                        "{RED}! {label} — attack failed: {}{RESET}",
                        first_line(&reason)
                    );
                    println!("\n{BOLD}Next up:{RESET} {label}");
                    println!("{BOLD}Progress: {solved}/5 solved{RESET}");
                    return Ok(());
                }
            }
        }};
    }

    play!(
        "Ackee 1 — Ancient Vault",
        async { level_1::Target::from_rpc(rpc).await },
        |target: level_1::Target| async move { target.check_rpc(rpc).await },
        hack1_ancient_vault::attack,
        None::<&str>
    );

    play!(
        "Ackee 2 — Monster Battle",
        async { level_2::Target::from_rpc(rpc, &actors.level_2).await },
        |target: level_2::Target| async move { target.check_rpc(rpc).await },
        hack2_monster_battle::attack,
        None::<&str>
    );

    play!(
        "Ackee 3 — Hidden Faction",
        async { level_3::Target::from_rpc(rpc, &actors.level_3).await },
        |target: level_3::Target| async move {
            target
                .check_rpc(
                    rpc,
                    hacker,
                    attack::solana::player_answer(3).as_deref(),
                )
                .await
        },
        hack3_hidden_faction::attack,
        Some(
            "reveal the faction secret, save it as plain text in \
             .solana/answers/ackee-level-3.txt, and rerun"
        )
    );

    play!(
        "Ackee 4 — Vesting",
        async { level_4::Target::from_rpc(rpc, &actors.level_4).await },
        |target: level_4::Target| async move {
            target
                .check_rpc(
                    rpc,
                    hacker,
                    attack::solana::player_answer(4).as_deref(),
                )
                .await
        },
        hack4_vesting::attack,
        Some(
            "reveal the secret, save it as plain text in \
             .solana/answers/ackee-level-4.txt, and rerun"
        )
    );

    play!(
        "Ackee 5 — Guardian Escrow",
        async {
            level_5::Target::from_rpc(rpc, &actors.level_5, &offender.pubkey())
                .await
        },
        |target: level_5::Target| async move {
            target
                .check_rpc(
                    rpc,
                    hacker,
                    attack::solana::player_answer(5).as_deref(),
                )
                .await
        },
        hack5_guardian_escrow::attack,
        Some(
            "reveal the secret, save it as plain text in \
             .solana/answers/ackee-level-5.txt, and rerun"
        )
    );

    println!(
        "{GREEN}{BOLD}All 5 Ackee levels solved ({solved}/5). Legends only.{RESET}"
    );
    Ok(())
}

async fn ethereum_track() -> eyre::Result<()> {
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

    #[test]
    fn learnings_cover_solved_levels_but_not_missing_notes() {
        let fallback = learnings_for("Fallback");
        assert!(fallback.is_some(), "Fallback ships upstream notes");
        assert!(!fallback.unwrap().trim().is_empty());
        assert!(learnings_for("Bet House").is_none(), "no upstream notes");
        assert!(learnings_for("Puppet").is_none(), "DVD ships no notes");
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
