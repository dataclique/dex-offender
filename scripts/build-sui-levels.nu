# Build every Sui CTF level package in the pinned submodules.
#
# lets-ctf's Move.toml pins the Sui framework through a gitee.com
# mirror that is unreachable from this network; the same repo+rev
# exists on github. The per-invocation git URL rewrite below redirects
# ONLY this command's fetches — no global config, no source change.

def main [] {
  with-env {
    GIT_CONFIG_COUNT: '1'
    GIT_CONFIG_KEY_0: 'url.https://github.com/MystenLabs/sui.git.insteadOf'
    GIT_CONFIG_VALUE_0: 'https://gitee.com/MystenLabs/sui.git'
  } {
    for pkg in [
      'lib/lets-ctf/src/01_check_in/check_in'
      'lib/lets-ctf/src/02_lets_move/lets_move'
      'ctf/fixtures/sui_smoke'
    ] {
      print $"building ($pkg)"
      cd $pkg
      let build = (sui move build | complete)
      cd -
      if ($build.exit_code? | default 1) != 0 {
        print ($build.stderr? | default ($build.stdout? | default ''))
        error make {msg: $"build failed for ($pkg)"}
      }
      print '  ok'
    }
  }
}
