//! plant_uniswap_v2_factory: canonical-address planting regression.
//! Verifies the planted factory is functional (createPair returns a
//! nonzero pair) and its constructor storage (feeToSetter at slot 1) is
//! initialized to the deployer.

use alloy::{primitives::address, providers::Provider, sol};
use ctf::{Roles, anvil};
use std::net::TcpStream;
use std::time::Duration;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Factory,
    "../lib/damn-vulnerable-defi/out/IUniswapV2Factory.sol/IUniswapV2Factory.json"
);

fn wait_for_port(port: u16) {
    let addr = format!("127.0.0.1:{port}");
    for _ in 0..200 {
        if TcpStream::connect(&addr).is_ok() {
            return;
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    panic!("anvil did not open {port}");
}

#[tokio::test]
async fn plants_factory_with_initialized_storage() {
    let mut anvil = std::process::Command::new("anvil")
        .args(["--port", "8912", "--code-size-limit", "50000"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("spawn anvil");
    wait_for_port(8912);

    let result = async {
        let roles = Roles::new("http://127.0.0.1:8912").expect("roles");
        let factory_addr = anvil::plant_uniswap_v2_factory(&roles).await?;
        let factory = Factory::new(factory_addr, roles.provider());

        // createPair must succeed (functional runtime code).
        let token_a = address!("0000000000000000000000000000000000000001");
        let token_b = address!("0000000000000000000000000000000000000002");
        let pair = factory.createPair(token_a, token_b).call().await?.0;
        assert!(!pair.is_zero(), "createPair must return nonzero pair");

        // feeToSetter (slot 1) must equal the deployer — constructor
        // storage was initialized during deploy, not skipped by planting
        // bare runtime code.
        let slot1 = roles
            .provider()
            .get_storage_at(factory_addr, alloy::primitives::U256::from(1))
            .await?;
        let as_addr = alloy::primitives::Address::from_word(
            alloy::primitives::B256::from(slot1.to_be_bytes()),
        );
        assert_eq!(
            as_addr,
            roles.deployer.address(),
            "feeToSetter storage must be initialized"
        );

        eyre::Ok(())
    }
    .await;

    anvil.kill().expect("kill anvil");
    anvil.wait().expect("wait anvil");

    result.expect("factory planting with initialized storage");
}
