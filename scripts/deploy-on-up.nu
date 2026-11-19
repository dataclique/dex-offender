# Started by `devenv up` next to the local chains: waits for the
# persistent Surfpool RPC, then runs the idempotent Solana deployment
# (existing chain state, solved levels, and identities are preserved).
# The Ethereum deployment deliberately does not run here: it is a full
# redeploy that overwrites state.json, so it stays behind the manual
# deploy-levels script.
def surfpool_healthy [] {
    try {
        http post -t application/json http://127.0.0.1:8899 '{"jsonrpc":"2.0","id":1,"method":"getHealth"}'
            | get result
            | str contains ok
    } catch {
        false
    }
}

mut ready = false
for _ in 1..120 {
    if (surfpool_healthy) {
        $ready = true
        break
    }
    sleep 500ms
}

if not $ready {
    print "Surfpool never became healthy at http://127.0.0.1:8899 within a minute; not deploying levels"
    exit 1
}

cargo run --locked -p ctf --bin deploy_levels -- solana
