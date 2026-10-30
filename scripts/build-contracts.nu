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
  let all = if ($roots | is-empty) {
    [
      lib/ethernaut/contracts
      lib/damn-vulnerable-defi
      lib/v3-core
      lib/v3-periphery
    ]
  } else {
    $roots
  }
  for $root in $all {
    build-root $root
  }
}
