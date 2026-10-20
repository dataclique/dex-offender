#!/usr/bin/env nu

# Build the pinned CTF submodules with forge and extract slim ABI JSON files
# into ctf/abi/<game>/<Contract>.json for alloy `sol!` bindings.
#
# Usage: nu scripts/build-abis.nu
# Requires: forge on PATH (devenv shell provides it).

# Artifact layout for current foundry: out/<Contract>.sol/<Contract>.json
def artifact-path [root: string, name: string] {
  $"($root)/out/($name).sol/($name).json"
}

# A valid forge ABI is a table of records, each with a nonempty string
# `type` field; an empty table is valid for artifacts like Force (no
# functions). Verified against a real forge artifact and against non-record /
# non-string-type / missing-type inputs.
def valid-abi [value] {
  let desc = ($value | describe)
  if not (($desc | str starts-with 'table') or ($desc | str starts-with 'list')) {
    return false
  }
  let bad = ($value | where {|e| (not (($e | describe) | str starts-with 'record')) or ((($e.type? | default '') | describe) != 'string') or (($e.type? | default '') == '') } | length)
  $bad == 0
}

# Pass through a forge artifact as the deployable artifact JSON sol!
# consumes (alloy-rs/examples' Counter.json uses the same shape: full
# artifact with `abi` and solc-standard-json `bytecode` {object, …}).
# Returns null when the artifact is missing/malformed: bad abi shape, or
# bytecode absent, or the bytecode object-string not 0x-prefixed.
export def artifact-to-abi-json [artifact] {
  let abi = ($artifact | get -o abi)
  let code = ($artifact.bytecode?.object? | default '')
  if not (valid-abi $abi) or ($code == '') or (($code | str substring 0..1) != '0x') {
    return null
  }
  $artifact | to json
}

def extract-game [root: string, game: string, contracts: list<string>] {
  mkdir $"ctf/abi/($game)"
  mut extracted = 0
  mut missing = []
  for $name in $contracts {
    let artifact_path = (artifact-path $root $name)
    if not ($artifact_path | path exists) {
      $missing = ($missing | append $name)
      continue
    }
    let json = (artifact-to-abi-json (open $artifact_path))
    if ($json == null) {
      $missing = ($missing | append $name)
      continue
    }
    $json | save -f $"ctf/abi/($game)/($name).json"
    $extracted = ($extracted + 1)
  }
  print $"($game): extracted ($extracted)/($contracts | length) ABIs"
  if not ($missing | is-empty) {
    error make {msg: $"($game): missing or malformed ABI artifacts for: ($missing | str join ', ')"}
  }
}

def main [] {
  print 'Building lib/ethernaut/contracts with forge...'
  let build1 = (do { forge build --root lib/ethernaut/contracts } | complete)
  if $build1.exit_code != 0 {
    print $build1.stderr
    error make {msg: 'forge build failed for lib/ethernaut/contracts'}
  }
  let ethernaut_levels = [
    Fallback Fallout CoinFlip Telephone Token Delegation Force Vault King Reentrance
    Elevator Privacy GatekeeperOne GatekeeperTwo NaughtCoin Preservation Recovery
    MagicNum AlienCodex Denial Shop Dex DexTwo PuzzleWallet Motorbike
    DoubleEntryPoint GoodSamaritan GatekeeperThree Switch HigherOrder Stake
    Impersonator MagicAnimalCarousel BetHouse EllipticToken Cashback
    ImpersonatorTwo UniqueNFT Forger NotOptimisticPortal
  ]
  extract-game lib/ethernaut/contracts ethernaut $ethernaut_levels

  print 'Building lib/damn-vulnerable-defi with forge...'
  let build2 = (do { forge build --root lib/damn-vulnerable-defi } | complete)
  if $build2.exit_code != 0 {
    print $build2.stderr
    error make {msg: 'forge build failed for lib/damn-vulnerable-defi'}
  }
  let dvd_levels = [
    UnstoppableVault ReceiverUnstoppable NaiveReceiverLenderPool
    FlashLoanReceiver TrusterLenderPool SideEntranceLenderPool
    TheRewarderPool RewardToken AccountingToken FlashLoanerPool SelfiePool
    SimpleGovernance DamnValuableTokenSnapshot TrustfulOracle Exchange
    PuppetPool PuppetV2Pool FreeRiderNFTMarketplace FreeRiderRecovery
    WalletRegistry ClimberTimelock ClimberTimelockBase ClimberVault
    AuthorizerUpgradeable WalletDeployer PuppetV3Pool AuthorizedExecutor
    SelfAuthorizedExecutor ShardsNFTMarketplace ShardsFeeVault
    CurvyPuppetLending L1Gateway L1Forwarder L2Handler L2MessageStore
    TokenBridge DamnValuableStaking DamnValuableToken DamnValuableNFT
  ]
  extract-game lib/damn-vulnerable-defi damn-vulnerable-defi $dvd_levels
}
