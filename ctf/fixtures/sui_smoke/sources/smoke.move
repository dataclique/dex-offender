#[allow(lint(self_transfer))]
module sui_smoke::smoke;

/// Minimal publish round-trip fixture: mints a smokable object and
/// transfers it to the transaction sender. Exists purely to prove the
/// harness can publish Move code on localnet and read back a concrete
/// owned object produced by the resulting module call.
public struct Smoke has key, store {
    id: UID,
}

public fun mint(ctx: &mut TxContext): Smoke {
    Smoke { id: object::new(ctx) }
}

public fun mint_and_transfer(ctx: &mut TxContext) {
    let smoke = mint(ctx);
    transfer::public_transfer(smoke, ctx.sender());
}
