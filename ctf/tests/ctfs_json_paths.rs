//! The practice runner reads the saved board from the repository root.
//! This test binary runs in its own process, so switching its working
//! directory cannot interfere with other integration tests.

use std::{env, path::Path};

#[test]
fn loads_board_from_repository_root() -> eyre::Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    if !root.join("ctfs.json").exists() && !crate_dir.join("ctfs.json").exists()
    {
        // Fresh clone: nothing is deployed yet, so there is no board to
        // load from either location. Run deploy_levels first.
        return Ok(());
    }

    let previous = env::current_dir()?;
    env::set_current_dir(root)?;
    let result = ctf::CTFs::from_file();
    env::set_current_dir(previous)?;
    result?;
    Ok(())
}
