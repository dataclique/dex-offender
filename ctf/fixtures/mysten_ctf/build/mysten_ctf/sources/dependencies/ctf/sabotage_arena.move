module ctf::sabotage_arena;

use sui::table::{Self, Table};
use sui::clock::Clock;
use ctf::flag;

const EAlreadyRegistered: u64 = 0;
const ENotRegistered: u64 = 1;
const ECooldownActive: u64 = 2;
const EShieldBelowThreshold: u64 = 3;
const ENoFlagsRemaining: u64 = 4;
const EArenaClosed: u64 = 5;
const ECannotAttackSelf: u64 = 6;

public struct AdminCap has key, store {
  id: UID,
}

public struct PlayerState has store {
  shield: u64,
  last_action_ms: u64,
}

public struct Arena has key {
  id: UID,
  players: Table<address, PlayerState>,
  flags_remaining: u8,
  shield_threshold: u64,
  cooldown_ms: u64,
  deadline_ms: u64,
}

fun init(ctx: &mut TxContext) {
  transfer::share_object(Arena {
    id: object::new(ctx),
    players: table::new(ctx),
    flags_remaining: 10,
    shield_threshold: 12,
    cooldown_ms: 600_000,
    deadline_ms: 1772974800000, 
  });

  transfer::transfer(AdminCap {
    id: object::new(ctx),
  }, ctx.sender());
}

public fun register(arena: &mut Arena, clock: &Clock, ctx: &mut TxContext) {
  assert!(clock.timestamp_ms() < arena.deadline_ms, EArenaClosed);
  let sender = ctx.sender();
  assert!(!arena.players.contains(sender), EAlreadyRegistered);

  arena.players.add(sender, PlayerState {
    shield: 0,
    last_action_ms: 0,
  });
}

public fun build(arena: &mut Arena, clock: &Clock, ctx: &mut TxContext) {
  assert!(clock.timestamp_ms() < arena.deadline_ms, EArenaClosed);
  let sender = ctx.sender();
  assert!(arena.players.contains(sender), ENotRegistered);

  let now = clock.timestamp_ms();
  let player = &mut arena.players[sender];
  assert!(now >= player.last_action_ms + arena.cooldown_ms, ECooldownActive);

  player.shield = player.shield + 1;
  player.last_action_ms = now;
}

public fun attack(arena: &mut Arena, target: address, clock: &Clock, ctx: &mut TxContext) {
  assert!(clock.timestamp_ms() < arena.deadline_ms, EArenaClosed);
  let sender = ctx.sender();
  assert!(sender != target, ECannotAttackSelf);
  assert!(arena.players.contains(sender), ENotRegistered);
  assert!(arena.players.contains(target), ENotRegistered);

  let now = clock.timestamp_ms();
  let attacker = &mut arena.players[sender];
  assert!(now >= attacker.last_action_ms + arena.cooldown_ms, ECooldownActive);
  attacker.last_action_ms = now;

  let target_state = &mut arena.players[target];
  target_state.shield = target_state.shield - 1;
}

public fun claim_flag(arena: &mut Arena, clock: &Clock, ctx: &mut TxContext): flag::Flag {
  assert!(clock.timestamp_ms() < arena.deadline_ms, EArenaClosed);
  let sender = ctx.sender();
  assert!(arena.players.contains(sender), ENotRegistered);
  assert!(arena.flags_remaining > 0, ENoFlagsRemaining);

  let player = &arena.players[sender];
  assert!(player.shield >= arena.shield_threshold, EShieldBelowThreshold);

  let PlayerState { .. } = arena.players.remove(sender);
  arena.flags_remaining = arena.flags_remaining - 1;

  flag::new(b"sabotage_arena".to_string(), ctx)
}

public fun admin_distribute(
  _: &AdminCap,
  arena: &mut Arena,
  recipient: address,
  ctx: &mut TxContext,
) {
  assert!(arena.players.contains(recipient), ENotRegistered);
  assert!(arena.flags_remaining > 0, ENoFlagsRemaining);

  let PlayerState { .. } = arena.players.remove(recipient);
  arena.flags_remaining = arena.flags_remaining - 1;

  transfer::public_transfer(flag::new(b"sabotage_arena".to_string(), ctx), recipient);
}
