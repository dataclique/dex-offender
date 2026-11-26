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
```

`devenv up` starts both practice chains: Anvil on `http://127.0.0.1:8545` from
the saved `state.json`, Surfpool on `http://127.0.0.1:8899` from the persistent
`.solana/surfpool` store, and it runs the idempotent Solana level deployment
once Surfpool is healthy. `cargo run -p ctf --bin deploy_levels` stays available
for manual deploys; `-- ethereum` or `-- solana` picks one chain.

`move-over-ctf` is initialized separately without recursion because its pinned
upstream revision contains an `openzeppelin-ui` gitlink without a URL. Its
challenge sources are preserved; the unused nested UI checkout is skipped.

Use direnv with nix-direnv. Without direnv, enter the environment manually with
`nix develop --impure`.

## Ethereum

The deployment command creates `ctfs.json` and `state.json`, then stops its
temporary Anvil. `devenv up` (or `devenv up anvil`) keeps your practice chain
running from that saved deployment; without it, start the chain yourself in a
separate terminal with `anvil --state state.json --code-size-limit 50000`.

The Ethernaut progression test connects to `http://127.0.0.1:8545`; it does not
start Anvil or redeploy levels. `--state` loads the saved deployment and writes
the updated state when Anvil exits, preserving completed levels between
sessions.

## Solana

`devenv up` (or `devenv up surfpool`) starts Surfpool on `http://127.0.0.1:8899`
from the persistent `.solana/surfpool` store in the checkout you run it from,
and deploys the Ackee levels once it is healthy. See the Solana levels section
for the underlying commands and the state/identity layout.

## Practice

Run the watcher from the repository (the track is explicit — there is no
default):

```sh
watch-levels ethereum
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
cargo run -- --track ethereum
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

Solana wargames live under `attack/src/solana/<game>/` — Ackee
(`ackee/hack1_ancient_vault.rs` … `hack5_guardian_escrow.rs`) and Paradigm 2022
(`paradigm2022/hack_hana.rs`). Each stub header names the goal and the pinned
upstream sources; no solutions ship.

### Localnet and deployment

The Solana track runs on a persistent local [Surfpool](https://surfpool.dev)
chain. Start it once in the background:

`devenv up` runs this for you (see Setup); the underlying process is:

```sh
surfpool start --offline --no-deploy --no-tui --no-studio \
  --host 127.0.0.1 --port 8899 --ws-port 8900 \
  --db .solana/surfpool --surfnet-id dex-offender
```

State lives in the checkout's `.solana/` and survives restarts, so only one
checkout can run the chain at a time. The `devenv up` deployment binds to the
offender identity and setup actors of the checkout it runs from. Manual deploys
use `deploy_levels` — no flag deploys both chains, `-- ethereum` or `-- solana`
picks one:

```sh
cargo run -p ctf --bin deploy_levels -- solana
```

This deploys all five Ackee levels at their canonical program ids (level 5
includes its verification program), creates your offender identity at
`.solana/offender.json` plus the levels' persistent setup identities at
`.solana/ackee-actors.json` (both created once, never replaced), and airdrops
what setup needs. Rerunning it is safe: deployed programs, solved state, and
identities are preserved. The `solana` CLI is not needed; point one at the chain
with `solana --url http://127.0.0.1:8899` if you want it.

### Playing

Pass `--track solana` and use the same `watch-levels solana` loop: every save
replays the current level against the persistent chain, with the same banners
and stop-at-first-unsolved gate. Progress is the chain state itself — your
solved levels stay solved across runs, and your identity persists through
`.solana/offender.json`.

Progression is Ackee 1-5 in order. Levels 3-5 end in revealing a secret: once
you have extracted it, save it as plain text in
`.solana/answers/ackee-level-N.txt` (N = 3, 4, or 5) and rerun — the runner
submits your secret to the level's program, which alone decides whether it is
right. Levels that need you to co-sign with setup identities (level 2's two
explorer users, level 3's faction authority) expose them on the level's target.
A level whose setup could not complete prints `Blocked at:` with the reason.

Inside `attack(target, offender)` — `offender` is your `SolActor`, whose
`rpc_client()` talks to the localnet and whose keypair signs:

```rust
use solana_sdk::instruction::{AccountMeta, Instruction};

let ix = Instruction::new_with_bincode(
    target.program_id,
    &discriminator_and_data, // Anchor 8-byte discriminator + args
    vec![AccountMeta::new(offender.pubkey(), true)],
);
let message = Message::new(&[ix], Some(&offender.pubkey()));
let tx = Transaction::new(
    &[offender],
    message,
    offender.rpc_client().get_latest_blockhash().await?,
);
offender.rpc_client().send_and_confirm_transaction(&tx).await?;
```

### Bindings for a level's program

Anchor programs get typed bindings two ways, both already wired:

**`anchor-client` (recommended):** the `attack` crate depends on
`anchor-client`. Build a typed program handle and call instructions with Rust
types, no manual discriminators:

```rust
use anchor_client::{Client, Cluster};
use ctf::solana::ackee::level_1::*; // re-exports the CPI types

let client = Client::new(Cluster::Localnet, offender.keypair.clone());
let program = client.program(target.program_id)?;
// program.request() builds typed instructions from the IDL
```

**CPI crate:** level 1's IDL is compiled into `ackee-level-1-cpi` under `cpi/`
(generated by `anchor-gen`), re-exported from the level module. Its types
(instruction args, account structs) work with both approaches. More levels get
CPI crates as they're verified.

For reference when neither exists: the IDL files ship in the submodule
(`lib/solana-auditors-bootcamp/Capture-the-Flag/level-N/target/idl/`),
instruction discriminators are `sha256("global:<method>")[..8]`, account
discriminators `sha256("account:<Struct>")[..8]`, and args are borsh (numbers
little-endian, strings/vecs as `u32` length prefix + bytes).

### Reading program state

Accounts are plain bytes: fetch and borsh-decode, skipping the 8-byte account
discriminator first.

```rust
use borsh::BorshDeserialize;

let raw = offender
    .rpc_client()
    .get_account(&vault_pda)
    .await?
    .ok_or_else(|| eyre::eyre!("vault account missing"))?;
let vault = Vault::deserialize(&mut &raw.data[8..])?;
```

Define `Vault` yourself with `#[derive(BorshDeserialize)]` mirroring the IDL
(field order matters).

### Writing program state

On Solana, state only changes through the program's own instructions — that's
the challenge path. Send them signed by your offender as shown above; extra
signers (e.g. a freshly created account's keypair) join the same transaction's
signer list.

### Deploying your own attack program

For these suites you don't need one: everything an exploit does — crafting
arbitrary instructions, signing as several actors, driving CPI order — happens
client-side in `attack()`. An on-chain helper would need an SBF toolchain, which
this repo deliberately doesn't ship (that absence is exactly what blocks
building otter-world/otterswap). If you ever do build a `.so` elsewhere,
Surfpool's `surfnet_writeProgram` RPC (the same one the deployer uses) loads it
at a fixed address on your localnet.

`hint!()` exists there too, but no Solana level has a registered hint yet. If
the runtime's Token-2022 rejects level 4's pinned CPI encoding, the deployer
prints the exact failure and the runner reports that level as blocked.

## Where things live

- `attack/src/ethereum/<game>/`: existing attack modules.
- `ctf/src/ethereum/<game>/`: Rust target types, Alloy bindings, setup and
  checks.
- `lib/<game>/`: pinned upstream contract sources and original challenge text.
- `lib/<game>/out/` (or the challenge's own `out/`): generated upstream Forge
  artifacts consumed directly by Alloy's `sol!` macro. These are not committed.
- `ctf/src/solana/`: persistent Surfpool targets (`ackee/`, `rpc.rs`) plus the
  LiteSVM harness still used by Paradigm 2022 hana.
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
