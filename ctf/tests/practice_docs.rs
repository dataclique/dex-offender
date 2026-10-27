//! Practice entry points must link to their matching upstream challenge.

use std::{fs, path::Path};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Challenge {
    deploy_id: String,
    instance_contract: String,
    description: String,
}

#[derive(serde::Deserialize)]
struct GameData {
    levels: Vec<Challenge>,
}

#[test]
fn first_ten_ethernaut_hacks_link_to_their_contracts_and_challenges()
-> eyre::Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let hacks = root.join("attack/src/ethereum/ethernaut");
    let game: GameData = serde_json::from_str(&fs::read_to_string(
        root.join("lib/ethernaut/client/src/gamedata/gamedata.json"),
    )?)?;
    let mut count = 0;

    for entry in fs::read_dir(&hacks)? {
        let path = entry?.path();
        let name = path.file_name().unwrap().to_string_lossy();
        if !name.starts_with("hack") || !name.ends_with(".rs") {
            continue;
        }
        let id: usize = name
            .get(4..6)
            .ok_or_else(|| eyre::eyre!("bad hack name: {name}"))?
            .parse()?;
        if id > 10 {
            continue;
        }
        count += 1;
        let challenge = game
            .levels
            .iter()
            .find(|level| level.deploy_id == id.to_string())
            .ok_or_else(|| {
                eyre::eyre!("{name}: no matching upstream challenge")
            })?;
        let source = fs::read_to_string(&path)?;

        for (label, relative) in [
            (
                "Source",
                format!("contracts/src/levels/{}", challenge.instance_contract),
            ),
            (
                "Challenge",
                format!(
                    "client/src/gamedata/en/descriptions/levels/{}",
                    challenge.description
                ),
            ),
        ] {
            let link = format!("../../../../lib/ethernaut/{relative}");
            let expected = format!("//! {label}: [local]({link})");
            eyre::ensure!(
                source.contains(&expected),
                "{name}: missing matching {label} link"
            );
            eyre::ensure!(
                hacks.join(&link).is_file(),
                "{name}: broken practice link {link}"
            );
            let upstream = format!(
                "https://github.com/OpenZeppelin/ethernaut/blob/d05643a40aa98c45d66247c69ffceab8f44dd8cb/{relative}"
            );
            eyre::ensure!(
                source.contains(&upstream),
                "{name}: missing pinned {label} link"
            );
        }
        eyre::ensure!(
            source.contains("//! Goal:"),
            "{name}: missing challenge goal"
        );
    }
    eyre::ensure!(
        count == 10,
        "expected the first 10 Ethernaut practice modules, found {count}"
    );
    Ok(())
}
