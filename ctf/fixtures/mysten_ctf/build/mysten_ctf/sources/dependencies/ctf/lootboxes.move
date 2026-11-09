module ctf::lootboxes;

use sui::random::{Self, Random};
use sui::coin::{Self, Coin};
use usdc::usdc::USDC;
use ctf::flag;

public struct MaybeFlag has key, store {
  id: UID,
  maybe_flag: Option<flag::Flag>
}

const ENoFlag: u64 = 0;
const EInsufficientPayment: u64 = 1;

// 15 USDC with 6 decimals = 12_000_000
const REQUIRED_PAYMENT: u64 = 12_000_000;

#[allow(lint(public_random))]
public fun open_lootbox(payment: Coin<USDC>, r: &Random, ctx: &mut TxContext): MaybeFlag {
  assert!(coin::value(&payment) == REQUIRED_PAYMENT, EInsufficientPayment);
  
  transfer::public_transfer(payment, @0x8e8cae7791a93778800b88b6a274de5c32a86484593568d38619c7ea71999654);
  
  let mut generator = random::new_generator(r, ctx); // generator is a PRG
  let random_value = generator.generate_u32_in_range(1, 100);

  MaybeFlag { 
    id: object::new(ctx),
    maybe_flag: if (random_value % 4 == 0) {
      option::some(flag::new(b"lootbox".to_string(), ctx))
    } else {
      option::none()
    }
  }
}

/// Extracts a flag from a MaybeFlag object.
/// Aborts with ENoFlag if the MaybeFlag doesn't contain a flag.
/// Destroys the MaybeFlag object in the process.
public fun extract_flag(maybe_flag: MaybeFlag): flag::Flag {
  let MaybeFlag { id, mut maybe_flag } = maybe_flag;
  object::delete(id);
  
  assert!(maybe_flag.is_some(), ENoFlag);
  let flag = maybe_flag.extract();
  maybe_flag.destroy_none();
  flag
}
