# DEX OFFENDER

Smart-contract CTFs with a Rust practice harness using
[Alloy](https://github.com/alloy-rs/alloy). Write attacks in the existing
`attack/` modules. Fallback is the only supplied solution.

## Setup

From the repository root:

```sh
git submodule update --init --recursive --force
git -c submodule.lib/move-over-ctf.update=checkout submodule update --init lib/move-over-ctf
direnv allow
forge build
nu scripts/build-contracts.nu
cargo run -p ctf --bin deploy_levels
```

`move-over-ctf` is initialized separately without recursion because its pinned
upstream revision contains an `openzeppelin-ui` gitlink without a URL. Its
challenge sources are preserved; the unused nested UI checkout is skipped.

Use direnv with nix-direnv. Without direnv, enter the environment manually with
`nix develop --impure`.

## Ethereum

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

## Solana

```sh
# First start: load the prepared initial accounts into persistent storage.
surfpool start --offline --no-deploy \
  --db solana-state.sqlite --surfnet-id dex-offender

# Subsequent starts: resume progress without reapplying initial accounts.
surfpool start --offline --no-deploy \
  --db solana-state.sqlite --surfnet-id dex-offender
```

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

Every Ethernaut level carries its upstream hint out of sight: call `hint!();`
anywhere inside your `attack()` to print it when you want it.

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

## Solana levels

Solana wargames live under `attack/src/solana/<game>/` — Ackee Bootcamp
(`ackee/hack1_ancient_vault.rs` … `hack5_guardian_escrow.rs`) and Paradigm 2022
(`paradigm2022/hack_hana.rs`). Each stub header names the goal and the pinned
upstream sources; no solutions ship.

There is no chain to start and no board to deploy: every run spins an in-process
LiteSVM, deploys the pinned `.so` programs, and funds the three roles fresh.
Play with:

```sh
cargo test -p attack solana -- --nocapture
```

Like the EVM gate, it plays in order (Bootcamp 1–5, then hana) and stops at the
first unsolved level. Progress isn't persisted — your stubs are the progress:
once `attack()` solves a level, every later run replays through it.

Inside `attack(target, &roles)` you have the same three roles as on EVM
(`roles.deployer`, `roles.some_user`, `roles.offender`, 1000 SOL each):

```rust
use solana_sdk::instruction::{AccountMeta, Instruction};

let ix = Instruction::new_with_bincode(
    target.program_id,
    &discriminator_and_data, // Anchor 8-byte discriminator + args
    vec![AccountMeta::new(roles.offender.pubkey(), true)],
);
let logs = roles.offender.send(&[ix])?; // program logs on success
```

### Bindings for a level's program

There is no Anchor client dependency in this workspace — bindings are
hand-rolled from the level's IDL, which ships in the submodule:
`lib/solana-auditors-bootcamp/Capture-the-Flag/level-N/target/idl/level_N.json`.
The IDL lists every instruction's args and accounts, and every account's fields
and PDA seeds. Three rules do the rest:

- instruction discriminator: `sha256("global:<method>")[..8]` — e.g.
  `sha256("global:initialize")[..8]`, prepended to the borsh-encoded args;
- account discriminator: `sha256("account:<Struct>")[..8]` — the first 8 bytes
  of every Anchor account's data;
- args are borsh: numbers little-endian, strings/vecs as `u32` length prefix
  - bytes.

```rust
use solana_sdk::hash::hash;

fn ix_discriminator(method: &str) -> [u8; 8] {
    let h = hash(format!("global:{method}").as_bytes());
    let mut d = [0u8; 8];
    d.copy_from_slice(&h.as_ref()[..8]);
    d
}

let mut data = ix_discriminator("initialize").to_vec();
data.extend_from_slice(&(name.len() as u32).to_le_bytes());
data.extend_from_slice(name.as_bytes());
```

For anything beyond strings, derive the arg struct with the workspace's `borsh`
crate (`#[derive(borsh::BorshSerialize)]`) and append `&args.try_to_vec()?`
after the discriminator.

### Reading program state

Accounts are plain bytes: fetch and borsh-decode, skipping the 8-byte account
discriminator first.

```rust
use borsh::BorshDeserialize;

let raw = roles
    .chain
    .get_account(&vault_pda)
    .ok_or_else(|| eyre::eyre!("vault account missing"))?;
let vault = Vault::deserialize(&mut &raw[8..])?;
```

Define `Vault` yourself with `#[derive(BorshDeserialize)]` mirroring the IDL
(field order matters).

### Writing program state

On Solana, state only changes through the program's own instructions — that's
the challenge path. Send them as any role; when an instruction creates an
account, its keypair co-signs alongside the payer via
`roles.chain.send_signed(&[&payer, &new_account], &[ix])?`. The harness also
exposes `chain.set_account` (a raw override) — that exists for level plumbing
and tests, not for solving.

### Deploying your own attack program

For these suites you don't need one: everything an exploit does — crafting
arbitrary instructions, signing as several actors, driving CPI order — happens
client-side in `attack()`. An on-chain helper would need an SBF toolchain, which
this repo deliberately doesn't ship (that absence is exactly what blocks
building otter-world/otterswap). If you ever do build a `.so` elsewhere,
`roles.chain.deploy_program(id, &elf)` loads it into the in-process SVM under
any keypair you choose, and it becomes callable like any other program.

`hint!()` exists there too, but no Solana level has a registered hint yet. Level
4 (Vesting) is partially playable — its header documents the exact runtime
blocker.

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
