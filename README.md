# DEX OFFENDER

Smart-contract CTFs with a Rust practice harness using
[Alloy](https://github.com/alloy-rs/alloy). Write attacks in the existing
`attack/` modules. Fallback is the only supplied solution.

## Start hacking Ethernaut

From the repository root, populate the pinned challenge sources and load the
development environment:

```sh
git submodule update --init --recursive
direnv allow
```

Use direnv with nix-direnv. Without direnv, enter the environment manually with
`nix develop --impure`.

Run a plain local chain in one terminal:

```sh
anvil
```

In another terminal, open this repository so direnv loads its environment, then
run the example:

```sh
cargo test -p attack fallback -- --nocapture
```

**Do not run `deploy_levels` or load `state.json` for these Ethernaut practice
tests.** Each test deploys its own fresh instance on the running Anvil at
`http://127.0.0.1:8545`.

## Work on a level

1. Open its existing `attack/src/ethereum/ethernaut/hackNN_name.rs` module.
2. Read the goal and open the direct contract-source URL in that module's
   header.
3. Implement `Exploit::attack` in the same file.
4. Run the test command shown in the header.

The first ten Ethernaut levels have named practice tests. For example, after
implementing the existing Fallout stub:

```sh
cargo test -p attack fallout -- --ignored --nocapture
```

Unfinished attacks are ignored by default. Selecting one with `--ignored` runs
its real success check, which fails until the attack solves the level. You do
not need to create another hack module or wire another test for these levels.

`target` contains the deployed instance address and any helper addresses.
`offender` is your player account. Use its provider and
`.from(offender.address())` when sending transactions. Do not use real keys or
funds.

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
