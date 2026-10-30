//! Damn Vulnerable DeFi — CurvyPuppet.
//!
//! Fork-only: seizes mainnet state at block 20190356 (Curve stETH/ETH
//! pool, permit2, WETH, and the LP whale). Run deploy_levels with
//! `FORK_URL` (and `FORK_BLOCK=20190356`) set.

use crate::{Level, Roles, anvil, to_ether};
use alloy::{
    primitives::{Address, U256, address},
    providers::Provider,
    sol,
    sol_types::SolCall,
};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    DamnValuableToken,
    "../lib/damn-vulnerable-defi/out/DamnValuableToken.sol/DamnValuableToken.json"
);

sol!(
    #[sol(rpc)]
    Weth,
    "../lib/damn-vulnerable-defi/out/WETH.sol/WETH.json"
);

sol!(
    #[sol(rpc)]
    CurvyPuppetOracle,
    "../lib/damn-vulnerable-defi/out/CurvyPuppetOracle.sol/CurvyPuppetOracle.json"
);

sol!(
    #[sol(rpc)]
    CurvyPuppetLending,
    "../lib/damn-vulnerable-defi/out/CurvyPuppetLending.sol/CurvyPuppetLending.json"
);

sol!(
    #[sol(rpc)]
    interface IErc20 {
        function transfer(address to, uint256 amount) external returns (bool);
        function approve(address spender, uint256 amount) external returns (bool);
        function balanceOf(address account) external view returns (uint256);
    }
);

sol!(
    #[sol(rpc)]
    interface IPermit2 {
        function approve(
            address token,
            address spender,
            uint160 amount,
            uint48 expiration
        ) external;
    }
);

const CURVE_POOL: Address =
    address!("DC24316b9AE028F1497c275EB9192a3Ea0f67022");
const PERMIT2: Address = address!("000000000022D473030F116dDEE9F6B43aC78BA3");
const MAINNET_WETH: Address =
    address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
const LP_WHALE: Address = address!("4F48031B0EF8acCea3052Af00A3279fbA31b50D8");
const NATIVE_ETH: Address =
    address!("EeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE");

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub address: Address,
    pub treasury: Address,
}

#[async_trait]
impl Level for Target {
    fn name(&self) -> &'static str {
        "CurvyPuppet"
    }

    fn from_file() -> eyre::Result<Self> {
        let ctfs = crate::CTFs::from_file()?;
        let dvd = ctfs
            .ethereum
            .damn_vulnerable_defi
            .curvy_puppet
            .as_ref()
            .ok_or_else(|| eyre::eyre!("level not yet deployed"))?;
        Ok(Target { address: dvd.address, treasury: dvd.treasury })
    }

    async fn set_up(roles: &Roles) -> eyre::Result<Self> {
        let deployer = &roles.deployer;
        let treasury = &roles.some_user;
        let player = &roles.offender;
        let provider = roles.provider();

        let pool_code = provider.get_code_at(CURVE_POOL).await?;
        if pool_code.is_empty() {
            return Err(eyre::eyre!(
                "CurvyPuppet needs a mainnet fork (set FORK_URL, FORK_BLOCK=20190356)"
            ));
        }

        println!("Deploying the CurvyPuppet challenge...");
        let dvt = DamnValuableToken::deploy(provider).await?;

        let oracle = CurvyPuppetOracle::deploy(provider).await?;
        // The oracle caps expirations at block.timestamp + 2 days.
        let now = provider.get_block_number().await?;
        let block = provider
            .get_block_by_number(now.into())
            .await?
            .ok_or_else(|| eyre::eyre!("missing block {now}"))?;
        let expiration =
            U256::from(block.header.timestamp) + U256::from(86_400);
        oracle
            .setPrice(NATIVE_ETH, to_ether(4000), expiration)
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;
        oracle
            .setPrice(*dvt.address(), to_ether(10), expiration)
            .from(deployer.address())
            .send()
            .await?
            .watch()
            .await?;

        let lending = CurvyPuppetLending::deploy(
            provider,
            *dvt.address(),
            CURVE_POOL,
            PERMIT2,
            *oracle.address(),
        )
        .await?;

        // Treasury wraps ETH into WETH and approves the player. On a fork
        // the anvil prefund covers the 200 WETH plus gas.
        let weth = Weth::new(MAINNET_WETH, provider.clone());
        weth.deposit()
            .value(to_ether(200))
            .from(treasury.address())
            .send()
            .await?
            .watch()
            .await?;
        weth.approve(player.address(), to_ether(200))
            .from(treasury.address())
            .send()
            .await?
            .watch()
            .await?;

        // LP tokens: impersonate the mainnet whale exactly like upstream
        // (vm.deal-style gas top-up first; the whale's real ETH is tiny).
        // The borrow/LP asset is the pool's lp_token(), not the pool.
        let lp_token = lending.borrowAsset().call().await?;
        anvil::impersonate(roles, LP_WHALE).await?;
        anvil::set_balance(roles, LP_WHALE, to_ether(1)).await?;
        anvil::send_as(
            roles,
            LP_WHALE,
            lp_token,
            IErc20::transferCall {
                to: *lending.address(),
                amount: to_ether(1000),
            }
            .abi_encode()
            .into(),
        )
        .await?;
        anvil::send_as(
            roles,
            LP_WHALE,
            lp_token,
            IErc20::transferCall {
                to: treasury.address(),
                amount: U256::from(65) * U256::from(10).pow(U256::from(15)),
            }
            .abi_encode()
            .into(),
        )
        .await?;

        // Three synthetic users open collateral positions (upstream
        // alice/bob/charlie; identities are irrelevant to the mechanics).
        for user in [
            address!("A11CE00000000000000000000000000000000000"),
            address!("B0B0000000000000000000000000000000000000"),
            address!("C0C0000000000000000000000000000000000000"),
        ] {
            dvt.transfer(user, to_ether(2500))
                .from(deployer.address())
                .send()
                .await?
                .watch()
                .await?;
            anvil::impersonate(roles, user).await?;
            anvil::set_balance(roles, user, to_ether(1)).await?;
            anvil::send_as(
                roles,
                user,
                *dvt.address(),
                IErc20::approveCall { spender: PERMIT2, amount: U256::MAX }
                    .abi_encode()
                    .into(),
            )
            .await?;
            anvil::send_as(
                roles,
                user,
                PERMIT2,
                IPermit2::approveCall {
                    token: *dvt.address(),
                    spender: *lending.address(),
                    amount: alloy::primitives::aliases::U160::MAX,
                    expiration: alloy::primitives::aliases::U48::MAX,
                }
                .abi_encode()
                .into(),
            )
            .await?;
            anvil::send_as(
                roles,
                user,
                *lending.address(),
                CurvyPuppetLending::depositCall { amount: to_ether(2500) }
                    .abi_encode()
                    .into(),
            )
            .await?;
            anvil::send_as(
                roles,
                user,
                *lending.address(),
                CurvyPuppetLending::borrowCall { amount: to_ether(1) }
                    .abi_encode()
                    .into(),
            )
            .await?;
        }

        println!("Deployed the CurvyPuppet lending at {}", lending.address());
        Ok(Target { address: *lending.address(), treasury: treasury.address() })
    }

    async fn check(&self, roles: &Roles) -> eyre::Result<bool> {
        let treasury = self.treasury;
        let player = roles.offender.address();
        let provider = roles.provider();
        let lending = CurvyPuppetLending::new(self.address, provider.clone());
        let dvt = IErc20::new(
            lending.collateralAsset().call().await?,
            provider.clone(),
        );

        println!(
            "Checking that all positions are closed and the treasury swept..."
        );
        // Mirror the upstream `_isSolved` conditions.
        for user in [
            address!("A11CE00000000000000000000000000000000000"),
            address!("B0B0000000000000000000000000000000000000"),
            address!("C0C0000000000000000000000000000000000000"),
        ] {
            if lending.getCollateralAmount(user).call().await? != U256::ZERO
                || lending.getBorrowAmount(user).call().await? != U256::ZERO
            {
                return Ok(false);
            }
        }
        Ok(
            // Treasury keeps the users' DVT and still holds WETH + LP.
            dvt.balanceOf(treasury).call().await? == to_ether(7500)
                && dvt.balanceOf(player).call().await? == U256::ZERO,
        )
    }
}
