//! Solana solution scaffolds, one directory per wargame (mirroring the
//! EVM layout under `attack/src/ethereum/`).
//!
//! ## Running levels
//!
//! Unlike the EVM board, Solana levels run on an in-process LiteSVM
//! chain created fresh for every run — there is no saved chain state.
//! `cargo test -p attack solana -- --nocapture` plays the wargames in
//! order (Ackee Bootcamp 1–5, then Paradigm 2022 hana) and stops at the
//! first unsolved level, just like the Ethernaut progression gate. Your
//! solutions live in these stubs, so every later run replays them.
//!
//! ## Writing an attack
//!
//! `ctf::solana` provides the same three roles as the EVM harness
//! (`SolRoles`: deployer, some_user, offender — each funded 1000 SOL).
//! Each `SolActor` sends transactions with its own keypair, e.g.:
//!
//! ```ignore
//! use solana_sdk::instruction::{AccountMeta, Instruction};
//!
//! let ix = Instruction::new_with_bincode(
//!     target.program_id,
//!     &discriminator_and_data, // Anchor 8-byte discriminator + args
//!     vec![AccountMeta::new(roles.offender.pubkey(), true)],
//! );
//! roles.offender.send(&[ix])?; // returns program logs on success
//! ```
//!
//! Raw account bytes come back through `roles.chain.get_account(&pubkey)`
//! (deserialize the level's Anchor accounts yourself, or parse the
//! fields the check reads). PDAs are derived with
//! `Pubkey::find_program_address(&[seeds], &program_id)`; each level's
//! `Target` carries the addresses its check observes. Solana levels have
//! no registered hints — `hint!()` reports that.

pub mod ackee;
pub mod paradigm2022;

#[cfg(test)]
mod practice {
    use crate::solana::{ackee, paradigm2022};
    use ctf::solana::{LiteSvmChain, SolChain, SolRoles, bootcamp, hana};
    use std::sync::Arc;

    /// Set up on a fresh chain, then either observe the already-solved
    /// state or run the player's attack and require it to solve the
    /// level. The first unsolved level aborts the whole progression.
    macro_rules! play {
        ($label:expr, $chain:expr, $set_up:expr, $attack:expr) => {{
            let chain: Arc<dyn SolChain> = $chain;
            let roles = SolRoles::new(chain.clone());
            let target = $set_up(&chain, &roles)?;
            if target.check(&chain) {
                println!("✓ {} — already solved", $label);
            } else {
                $attack(&target, &roles).await?;
                eyre::ensure!(
                    target.check(&chain),
                    "{}: attack ran but the level is not solved — stopping here",
                    $label
                );
                println!("✓ {} — solved just now", $label);
            }
        }};
    }

    #[tokio::test]
    async fn solana_progression() -> eyre::Result<()> {
        play!(
            "Ackee Bootcamp 1 — Ancient Vault",
            Arc::new(LiteSvmChain::new()),
            bootcamp::level_1::Target::set_up,
            ackee::hack1_ancient_vault::attack
        );

        play!(
            "Ackee Bootcamp 2 — Monster Battle",
            Arc::new(LiteSvmChain::new()),
            bootcamp::level_2::Target::set_up,
            ackee::hack2_monster_battle::attack
        );

        play!(
            "Ackee Bootcamp 3 — Hidden Faction",
            Arc::new(LiteSvmChain::new()),
            bootcamp::level_3::Target::set_up,
            ackee::hack3_hidden_faction::attack
        );

        play!(
            "Ackee Bootcamp 4 — Vesting",
            Arc::new(LiteSvmChain::new()),
            bootcamp::level_4::Target::set_up,
            ackee::hack4_vesting::attack
        );

        play!(
            "Ackee Bootcamp 5 — Guardian Escrow",
            Arc::new(LiteSvmChain::new()),
            bootcamp::level_5::Target::set_up,
            ackee::hack5_guardian_escrow::attack
        );

        // hana's prebuilt program is a legacy (solana 1.9-era) binary.
        play!(
            "Paradigm 2022 — hana",
            Arc::new(LiteSvmChain::with_legacy_programs()),
            hana::Target::set_up,
            paradigm2022::hack_hana::attack
        );

        println!("All Solana levels solved");
        Ok(())
    }
}
