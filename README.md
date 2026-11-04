# DEX OFFENDER

Smart-contract CTFs with a Rust practice harness using
[Alloy](https://github.com/alloy-rs/alloy). Write attacks in the existing
`attack/` modules. Fallback is the only supplied solution.

## Setup

From the repository root:

```sh
git submodule update --init --recursive
direnv allow
forge build
nu scripts/build-contracts.nu
cargo run -p ctf --bin deploy_levels
```

Use direnv with nix-direnv. Without direnv, enter the environment manually with
`nix develop --impure`.

The deployment command creates `ctfs.json` and `state.json`, then stops its
temporary Anvil. Start your practice chain from that saved deployment in a
separate terminal:

```sh
anvil --state state.json --code-size-limit 50000
```

Keep it running. The Ethernaut progression test connects to
`http://127.0.0.1:8545`; it does not start Anvil or redeploy levels. `--state`
loads the saved deployment and writes the updated state when Anvil exits,
preserving completed levels between sessions.

## Practice

Run the watcher from the repository:

```sh
watch-levels
```

That is `cargo watch -x run` (plus the path list that makes saves in hidden Git
worktrees count). Every save reruns the level runner against your chain:

- levels you already solved print `✓ already solved` and skip their attacks;
- a level you just solved gets the full `CONGRATULATIONS` banner, in green;
- the run stops at the first unsolved or failed level and prints `Next up:`; an
  uncheckable level prints `Blocked at:` instead. Later levels are not touched.

Progression is Ethernaut 1–40, then Paradigm 2023 Hello World, then Damn
Vulnerable DeFi. The runner prints `Progress: X/N solved` when it stops at a
level, or an all-solved message after completing every level. For a single run
without watching:

```sh
cargo run
```

It connects to your practice Anvil at `http://127.0.0.1:8545`; set
`ATTACK_RPC_URL` to point it elsewhere. If it cannot reach the chain it tells
you which command to start.

`cargo test -p attack -- --nocapture` runs Ethernaut as one sequential test that
fails at the first unsolved level — a strict progress gate. CI uses the runner
instead.

Open the existing `attack/src/ethereum/ethernaut/hackNN_name.rs` for the level
you want to solve. Its header gives the goal and direct contract-source URL.
Write your solution in `Exploit::attack`; it is already wired into the runner
and the test.

`target` contains the deployed instance address and any helper addresses.
`offender` is your player account. Its provider defaults to the offender: use
`offender.provider()` for deployments and calls. Explicit `.from(...)` overrides
are supported; `roles.provider()` defaults to the deployer for setup. Do not use
real keys or funds.

### Attack helper contracts

Need a Solidity contract for your attack (a fake `Building`, an exploit
contract, anything)? Write it under `attack/contracts/<game>/` and build it with
the root Foundry project:

```sh
forge build
```

The compiled artifact lands at `attack/out/<File>.sol/<Contract>.json`
(gitignored — your solutions stay yours). Import level contracts in your
Solidity via the root remappings:

```solidity
import "@ethernaut/Elevator.sol"; // lib/ethernaut/contracts/src/levels/
import "@dvd/puppet/PuppetPool.sol"; // lib/damn-vulnerable-defi/src/
```

Also available: `@paradigm-2021/`, `@paradigm-2022/`, `@paradigm-2023/`, plus
`@openzeppelin/contracts/` and the Uniswap/Safe libs from `lib/`.

In your Rust attack code, `sol!` paths are relative to the crate directory —
`attack/` — so the artifact path drops the `attack/` prefix:

```rust
use alloy::{sol, sol_types::SolContract};

sol!(
    #[sol(rpc)]
    Example,
    "out/Example.sol/Example.json"
);
```

(`forge build` first — the artifact must exist when `sol!` expands.) For the
already-deployed level contracts you usually need no `sol!` at all: the ctf
level modules re-export their bindings, e.g.
`use ctf::ethereum::ethernaut::lvl11_elevator::*;` gives you `Elevator`.

If anything reports `connection refused` on port 8545, start the saved-state
Anvil shown above. If you deploy a new set of levels, restart your practice
chain with the matching new `state.json` so its addresses agree with
`ctfs.json`.

## Where things live

- `attack/src/ethereum/<game>/`: existing attack modules.
- `ctf/src/ethereum/<game>/`: Rust target types, Alloy bindings, setup and
  checks.
- `lib/<game>/`: pinned upstream contract sources and original challenge text.
- `lib/<game>/out/` (or the challenge's own `out/`): generated upstream Forge
  artifacts consumed directly by Alloy's `sol!` macro. These are not committed.
- `ctf/src/solana/`: LiteSVM harnesses.
- `ctf/src/sui/`: experimental Sui integration, excluded from the default build.

Rust compilation requires the generated artifacts. The setup commands build
attack helpers with root `forge build` and upstream contracts in place with
`nu scripts/build-contracts.nu`; no artifact-copy step is needed. Rerun them
after updating submodule pins or contract sources. The build script also
attempts the unregistered Paradigm 2023 Token Locker project; its known
stack-too-deep failure is reported as a warning, not a successful build.

## Build and formatting

```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

The development shell installs formatter hooks configured directly in
`flake.nix`: Nix formatting, Rust formatting, Taplo for TOML and `deno fmt` for
Markdown. Keep pinned upstream sources in `lib/` unchanged.
