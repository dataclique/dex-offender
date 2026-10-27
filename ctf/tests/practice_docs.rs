//! Practice headers identify their contract without horizontal scrolling.

use std::{fs, path::Path};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Challenge {
    deploy_id: String,
    instance_contract: String,
}

#[derive(serde::Deserialize)]
struct GameData {
    levels: Vec<Challenge>,
}

#[test]
fn first_ten_ethernaut_headers_name_their_contracts_and_fit_the_editor()
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
        let relative = format!(
            "../../../../lib/ethernaut/contracts/src/levels/{}",
            challenge.instance_contract
        );
        eyre::ensure!(
            hacks.join(&relative).is_file(),
            "{name}: broken contract link"
        );

        let harness = root
            .join("ctf/src/ethereum/ethernaut")
            .join(name.replacen("hack", "lvl", 1));
        for file in [&path, &harness] {
            let source = fs::read_to_string(file)?;
            let header: Vec<_> = source
                .lines()
                .take_while(|line| line.starts_with("//!"))
                .collect();
            for line in &header {
                eyre::ensure!(
                    line.chars().count() <= 80,
                    "{}: header exceeds 80 columns",
                    file.display()
                );
            }
            let expected = format!("//! {relative}");
            eyre::ensure!(
                header.contains(&expected.as_str()),
                "{}: missing matching contract link",
                file.display()
            );
            eyre::ensure!(
                header.iter().any(|line| line.starts_with("//! Goal:")),
                "{}: missing goal",
                file.display()
            );
        }
    }
    eyre::ensure!(
        count == 10,
        "expected the first 10 Ethernaut practice modules, found {count}"
    );
    Ok(())
}
