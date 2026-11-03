#!/usr/bin/env nu

# Build the pinned CTF submodules with forge. Nothing is copied: alloy `sol!`
# bindings in ctf/src point straight at each submodule's out/ artifacts.
#
# Usage: nu scripts/build-contracts.nu
# Requires: forge on PATH (devenv shell provides it) and initialized
# submodules (git submodule update --init --recursive).

# 2021 challenges with their pinned upstream solc versions (build.sh).
const P2021_SOLC = {
  bank: '0.4.24'
  vault: '0.4.16'
  lockbox: '0.4.24'
  babysandbox: '0.7.0'
  market: '0.7.0'
  upgrade: '0.6.12'
  secure: '0.5.12'
  jop: '0.7.6'
  rever: '0.8.0'
  swap: '0.4.24'
  babyrev: '0.4.24'
  bouncer: '0.8.0'
  hello: '0.8.0'
  farmer: '0.8.0'
  yield_aggregator: '0.8.0'
  broker: '0.8.0'
}

def main [] {
  let p2023 = (glob 'lib/paradigm-ctf-2023/*/challenge/project')
  let p2022 = (glob 'lib/paradigm-ctf-2022/*/public' | where {|p|
    ($p | path join 'contracts' | path exists)
  })
  let p2021 = ($P2021_SOLC | columns | each {|c| $"lib/paradigm-ctf-2021/($c)/public" })
  let roots = (
    [lib/ethernaut/contracts lib/damn-vulnerable-defi]
    | append $p2023
    | append $p2022
    | append $p2021
  )

  mut failed = []
  for $root in $roots {
    print $"forge build --root ($root)"
    let solc = ($P2021_SOLC | get -o ($root | path dirname | path basename))
    let build = if ($root | str contains '/paradigm-ctf-2021/') {
      do { ^forge build --root $root --contracts contracts --remappings 'private=../private/' --remappings 'public=contracts/' --use $"solc:($solc)" } | complete
    } else if (($root | path join 'contracts') | path exists) {
      do { ^forge build --root $root --contracts contracts } | complete
    } else {
      do { ^forge build --root $root } | complete
    }
    print --no-newline $build.stdout
    print --stderr --no-newline $build.stderr
    if $build.exit_code != 0 {
      if ($root | path expand) == ('lib/paradigm-ctf-2023/token-locker/challenge/project' | path expand) {
        # No Rust binding uses this optional project; do not hide its failure.
        print --stderr $"WARNING: optional Token Locker build failed: ($root)"
      } else {
        $failed = ($failed | append $root)
      }
    }
  }
  if not ($failed | is-empty) {
    error make {msg: $"forge build failed for: ($failed | str join ', ')"}
  }
}
