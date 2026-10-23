# DEX OFFENDER

A compilation of smart contract CTF wargames with a Rust practice harness. Solve the challenges yourself in Rust using [alloy](https://github.com/alloy-rs/core) — no JavaScript required.

## Supported CTFs

| CTF                                                                            | Levels | Status                                                            |
| ------------------------------------------------------------------------------ | ------ | ----------------------------------------------------------------- |
| [Ethernaut](https://ethernaut.openzeppelin.org/)                               | 40     | ✅ 23 deployable, 17 scaffolds                                    |
| [Damn Vulnerable DeFi](https://damnvulnerabledefi.xyz/)                        | 15     | ✅ 14 deployable, 2 fork-deployable via `FORK_URL`                |
| [Paradigm CTF 2023](https://github.com/paradigmxyz/paradigm-ctf-2023)          | 14 EVM | ✅ 2 deployable, 12 scaffolds                                     |
| [Paradigm CTF 2021](https://github.com/paradigmxyz/paradigm-ctf-2021)          | 16 EVM | ✅ 1 deployable, 15 scaffolds                                     |
| [Paradigm CTF 2022](https://github.com/paradigmxyz/paradigm-ctf-2022)          | 12 EVM | ✅ scaffolds ready, deploy TODO                                   |
| [Ackee Bootcamp](https://github.com/Ackee-Blockchain/Solana-Auditors-Bootcamp) | 5      | ✅ Solana litesvm core ready                                      |
| [Mysten CTF](https://github.com/MystenLabs/CTF)                                | 5      | 🔍 research in progress                                           |
| [move_over_ctf](https://github.com/OpenZeppelin/move_over_ctf)                 | 8      | 🔍 research in progress                                           |
| [lets-ctf](https://github.com/hoh-zone/lets-ctf)                               | 2+     | 🔍 research in progress                                           |
| [neodyme solana-ctf](https://github.com/neodyme-labs/solana-ctf)               | 12     | 📋 submodule pinned                                               |
| [sealevel-attacks](https://github.com/coral-xyz/sealevel-attacks)              | 11     | 📋 [educational examples](lib/sealevel-attacks/) — not CTF levels |

## Quick start

```sh
# 1. Enter the dev shell
direnv allow

# 2. Deploy all levels to a local anvil
cargo run --bin deploy_levels

# 3. Start anvil with the saved state
anvil --load-state state.json

# 4. Run the example solution
cargo test -p attack -- --nocapture
```

## Solving a level

1. Find the level in `ctf/src/ethereum/<game>/lvlNN_name.rs`
2. Create your solution in `attack/src/ethereum/<game>/hackNN_name.rs`
3. Implement the `ctf::Exploit` trait's `attack` method
4. Wire it into the test in `attack/src/ethereum/<game>/mod.rs`
5. Run `cargo test -p attack -- --nocapture`

See `attack/src/ethereum/ethernaut/hack01_fallback.rs` for the ONLY example — all other hack files are intentionally blank.

## Architecture

```
ctf/src/
  ethereum/           # EVM chain harness
    ethernaut/        # Ethernaut levels
    damn_vulnerable_defi/  # DVD levels
    paradigm_2021/    # Paradigm 2021 levels
    paradigm_2022/    # Paradigm 2022 levels
    paradigm_2023/    # Paradigm 2023 levels
  solana/             # Solana chain harness (litesvm)
  roles.rs            # Actor/Roles (anvil keypairs)
  level.rs            # Level + Exploit traits

ctf/abi/              # ABI artifacts (forge-generated, consumed by sol!)
lib/                  # Pinned upstream CTF submodules
attack/src/           # Your solutions
```

## ABI bindings

ABI bindings are generated via alloy's `sol!` macro loading JSON artifacts from `ctf/abi/`. Regenerate after contract changes:

```sh
nu scripts/build-abis.nu
```

## Level status

- **Deployable**: `deploy_levels` successfully creates the challenge on local anvil
- **Scaffold**: module + ABI + hack stub exist but deployment or setup needs work
- **Fork-gated**: PuppetV3 + CurvyPuppet deploy via `FORK_URL`/`FORK_BLOCK`
  (verified with the free public `eth.drpc.org` archive endpoint;
  `ctf/tests/fork_levels.rs` runs them). electric-sheep stays
  fork-blocked pending its own setup port.

## Local blockchain

| Field    | Value                                                       |
| -------- | ----------------------------------------------------------- |
| Port     | 8545                                                        |
| Mnemonic | test test test test test test test test test test test junk |
| Offender | 0xa0Ee7A142d267C1f36714E4a8F75612F20a79720                  |
| Deployer | 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266                  |
