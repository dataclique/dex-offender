# Sui localnet lifecycle: guarded isolated start, JSON-RPC readiness
# wait, guaranteed owned teardown. Run from the dev shell (provides the
# pinned `sui` binary). Everything lives under .tmp/sui-localnet.

const sui_dir = ".tmp/sui-localnet"
const rpc_url = "http://127.0.0.1:9000"
const faucet_port = 9123

# curl exit 7 = connection refused (nothing listening); any other code
# means a listener exists.
def http_listening [port: int] {
  let res = (^curl -s -m 2 -o /dev/null $"http://127.0.0.1:($port)/" | complete)
  ($res.exit_code? | default 0) != 7
}

def rpc_ready [] {
  let res = (^curl -s -m 5 -X POST $rpc_url -H 'Content-Type: application/json' --data-binary '{"jsonrpc":"2.0","method":"sui_getChainIdentifier","params":[],"id":1}' | complete)
  if ($res.exit_code? | default 1) != 0 {
    return false
  }
  let parsed = (try { $res.stdout | from json } catch { return false })
  not ($parsed.result? | is-empty)
}

def stop_job [job: int] {
  try { job kill $job } catch { }
}

def main [] {
  # Guard: refuse if either localnet port is already taken.
  if (http_listening 9000) {
    print "port 9000 already in use; not starting localnet"
    return
  }
  if (http_listening $faucet_port) {
    print $"port ($faucet_port) already in use; not starting localnet"
    return
  }

  # Guard: reuse only an EMPTY scratch dir (dotfiles counted) — never
  # overwrite existing content; state persists across runs if kept.
  if ($sui_dir | path exists) {
    let non_empty = (ls -a $sui_dir | where {|e| (basename $e.name) not-in ['.' '..']} | length) > 0
    if $non_empty {
      print $"scratch ($sui_dir) has existing content; remove it manually first"
      return
    }
  } else {
    mkdir $sui_dir
  }

  # Generate an isolated network config first (sui start requires a
  # pre-generated network.yaml), then run the network from it.
  let gen = (^sui genesis --force --with-faucet --working-dir .tmp/sui-localnet | complete)
  if ($gen.exit_code? | default 1) != 0 {
    print $"sui genesis failed: ($gen.stderr? | default '')"
    return
  }

  print "starting sui localnet..."
  let job = (job spawn {
    sui start --network.config .tmp/sui-localnet out+err> .tmp/sui-localnet/start.log
  })
  print $'localnet job ($job)'

  try {
    let deadline = ((date now) + 180sec)
    loop {
      if (date now) > $deadline {
        error make {msg: 'localnet rpc never became ready'}
      }
      if (rpc_ready) { break }
      sleep 2sec
    }
    print "localnet rpc ready"
  } catch {|e|
    stop_job $job
    print $"teardown after failure: ($e.msg)"
    print "--- start.log tail ---"
    open --raw $"($sui_dir)/start.log" | lines | last 20 | str join (char newline) | print
    return
  }

  stop_job $job
  sleep 2sec
  if (http_listening 9000) {
    print "WARN: port 9000 still answering after job kill"
    return
  }
  rm -rf $sui_dir
  print "localnet torn down; port freed; scratch removed"
}
