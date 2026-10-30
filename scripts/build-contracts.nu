#!/usr/bin/env nu

# Build the pinned CTF submodules with forge. Nothing is copied: alloy `sol!`
# bindings in ctf/src point straight at each submodule's out/ artifacts.
#
# Usage: nu scripts/build-contracts.nu
# Requires: forge on PATH (devenv shell provides it) and initialized
# submodules (git submodule update --init --recursive).

def build-root [root: string] {
  print $"forge build --root ($root)"
  let build = (do { forge build --root $root } | complete)
  if $build.exit_code != 0 {
    print $build.stderr
    error make {msg: $"forge build failed for ($root)"}
  }
}

def main [...roots: string] {
  let p2023 = (glob 'lib/paradigm-ctf-2023/*/challenge/project')
  let all = if ($roots | is-empty) {
    [
      lib/ethernaut/contracts
      lib/damn-vulnerable-defi
    ] | append $p2023
  } else {
    $roots
  }
  mut failed = []
  for $root in $all {
    print $"forge build --root ($root)"
    let build = (do { forge build --root $root } | complete)
    if $build.exit_code != 0 {
      # token-locker (2023) still fails upstream with stack-too-deep.
      if not ($root | str contains 'token-locker') {
        $failed = ($failed | append $root)
      }
      print ($build.stderr | lines | last 1)
    }
  }
  if not ($failed | is-empty) {
    error make {msg: $"forge build failed for: ($failed | str join ', ')"}
  }
}
