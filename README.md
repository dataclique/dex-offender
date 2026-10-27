# DEX OFFENDER

Smart-contract CTFs with a Rust practice harness using
[Alloy](https://github.com/alloy-rs/alloy). Write attacks in the existing
`attack/` modules. Fallback is the only supplied solution.

## Setup

From the repository root:

```sh
git submodule update --init --recursive
direnv allow
cargo run --bin deploy_levels
```

Use direnv with nix-direnv. Without direnv, enter the environment manually with
`nix develop --impure`.

The deployment command creates `ctfs.json` and `state.json`, then stops its
temporary Anvil. Start your practice chain from that saved deployment in a
separate terminal:

```sh
anvil --state state.json --code-size-limit 50000
```

Keep it running. Tests connect to `http://127.0.0.1:8545`; they do not start
Anvil or redeploy levels. `--state` loads the saved deployment and writes the
updated state when Anvil exits, preserving completed levels between sessions.

## Practice

Run the watcher from the repository:

```sh
watch-levels
```

It reruns **all** registered Ethernaut tests when Rust sources change. Levels
already solved on your chain print `Already solved` and do not rerun their
attacks. Unsolved stubs fail their success checks until you implement them;
those failures are your progress list, not build errors. No tests are ignored.

The watcher explicitly watches `attack/src`, `ctf/src` and the Cargo manifests.
It uses polling and disables ignore-file filtering for those paths so saves in
hidden Git worktrees are detected, without watching build output or `.git/`.

For a single run without watching:

```sh
cargo test -p attack -- --nocapture
```

Open the existing `attack/src/ethereum/ethernaut/hackNN_name.rs` for the level
you want to solve. Its header gives the goal and direct contract-source URL.
Write your solution in `Exploit::attack`; the tests are already wired.

`target` contains the deployed instance address and any helper addresses.
`offender` is your player account. Use its provider and
`.from(offender.address())` when sending transactions. Do not use real keys or
funds.

If tests report `connection refused` on port 8545, start the saved-state Anvil
shown above. If you deploy a new set of levels, restart your practice chain with
the matching new `state.json` so its addresses agree with `ctfs.json`.

## Where things live

- `attack/src/ethereum/<game>/`: existing attack modules.
- `ctf/src/ethereum/<game>/`: Rust target types, Alloy bindings, setup and
  checks.
- `lib/<game>/`: pinned upstream contract sources and original challenge text.
- `ctf/abi/`: checked-in Forge artifacts consumed by Alloy's `sol!` macro. ABI
  JSON is not a replacement for reading the contract source.
- `ctf/src/solana/`: LiteSVM harnesses.
- `ctf/src/sui/`: experimental Sui integration, excluded from the default build.

The checked-in ABI artifacts are enough to build the default Rust workspace.
Regenerate them only when changing contract artifacts:

```sh
nu scripts/build-abis.nu
```

## Build and formatting

```sh
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

The development shell installs formatter hooks configured directly in
`flake.nix`: Nix formatting, Rust formatting, Taplo for TOML and `deno fmt` for
Markdown. Keep pinned upstream sources in `lib/` unchanged.
