//! lets-ctf check_in live deployment: publish the pinned package,
//! locate the shared FlagString from init, verify the fresh level
//! reads unsolved (ture_num == 0). Gated on SUI_LOCALNET_TEST.

use ctf::sui::SuiLocalnet;
use ctf::sui::lets_ctf::check_in;

#[tokio::test]
async fn deploys_check_in_unsolved() {
    if std::env::var("SUI_LOCALNET_TEST").is_err() {
        eprintln!("SUI_LOCALNET_TEST not set; skipping check_in live test");
        return;
    }

    let (localnet, roles) =
        SuiLocalnet::start_with_roles().await.expect("localnet with roles");
    let result = {
        async {
            let target = check_in::set_up(&localnet, &roles).await?;
            println!(
                "check_in package {} flag-string {}",
                target.package, target.flag_string
            );
            let solved = check_in::check(&target, &localnet).await?;
            Ok::<_, eyre::Report>(solved)
        }
        .await
    };
    let scratch = localnet.scratch_dir().to_path_buf();
    localnet.teardown().await.expect("teardown");

    let solved = result.expect("check_in setup+check");
    assert!(!solved, "fresh check_in must be unsolved (ture_num == 0)");
    assert!(!scratch.exists(), "scratch removed");
}
