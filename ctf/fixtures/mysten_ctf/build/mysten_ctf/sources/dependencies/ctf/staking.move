module ctf::staking;

use sui::coin::{Self, Coin};
use sui::sui::SUI;
use sui::balance::{Self, Balance};
use sui::clock::Clock;
use ctf::flag;

const MIN_STAKE_HOURS: u64 = 168;
const MIN_CLAIM_AMOUNT: u64 = 1_000_000_000;
const MILLISECONDS_PER_HOUR: u64 = 3_600_000;

const ENotEnoughStakingTime: u64 = 0;
const EInsufficientStakeAmount: u64 = 1;

public struct StakingPool has key {
  id: UID,
  balance: Balance<SUI>,
}

public struct StakeReceipt has key, store {
  id: UID,
  amount: u64,
  hours_staked: u64,
  last_update_timestamp: u64,
}

fun init(ctx: &mut TxContext) {
  transfer::share_object(StakingPool {
    id: object::new(ctx),
    balance: balance::zero(),
  });
}

public fun stake(
  pool: &mut StakingPool,
  coin: Coin<SUI>,
  clock: &Clock,
  ctx: &mut TxContext
): StakeReceipt {
  let amount = coin.value();
  
  balance::join(&mut pool.balance, coin::into_balance(coin));
  
  StakeReceipt {
    id: object::new(ctx),
    amount,
    hours_staked: 0,
    last_update_timestamp: clock.timestamp_ms(),
  }
}

public fun update_receipt(
  receipt: StakeReceipt,
  clock: &Clock,
  ctx: &mut TxContext
): StakeReceipt {
  let current_time = clock.timestamp_ms();
  let time_passed_ms = current_time - receipt.last_update_timestamp;
  let hours_passed = time_passed_ms / MILLISECONDS_PER_HOUR;
  
  let StakeReceipt { id, amount, hours_staked, last_update_timestamp: _ } = receipt;
  object::delete(id);
  
  StakeReceipt {
    id: object::new(ctx),
    amount,
    hours_staked: hours_staked + hours_passed,
    last_update_timestamp: current_time,
  }
}

public fun merge_receipts(
  r1: StakeReceipt,
  r2: StakeReceipt,
  clock: &Clock,
  ctx: &mut TxContext
): StakeReceipt {
  let StakeReceipt { 
    id: id1, 
    amount: amt1, 
    hours_staked: hours1,
    last_update_timestamp: _ 
  } = r1;
  
  let StakeReceipt { 
    id: id2, 
    amount: amt2, 
    hours_staked: hours2,
    last_update_timestamp: _ 
  } = r2;
  
  object::delete(id1);
  object::delete(id2);
  
  StakeReceipt {
    id: object::new(ctx),
    amount: amt1 + amt2,
    hours_staked: hours1 + hours2,
    last_update_timestamp: clock.timestamp_ms(),
  }
}

public fun claim_flag(
  pool: &mut StakingPool,
  receipt: StakeReceipt,
  clock: &Clock,
  ctx: &mut TxContext
): (flag::Flag, Coin<SUI>) {
  let current_time = clock.timestamp_ms();
  let time_passed_ms = current_time - receipt.last_update_timestamp;
  let hours_passed = time_passed_ms / MILLISECONDS_PER_HOUR;
  let total_hours = receipt.hours_staked + hours_passed;
  
  assert!(total_hours >= MIN_STAKE_HOURS, ENotEnoughStakingTime);
  assert!(receipt.amount >= MIN_CLAIM_AMOUNT, EInsufficientStakeAmount);
  
  let amount = receipt.amount;
  let staked_balance = balance::split(&mut pool.balance, amount);
  let staked_coin = coin::from_balance(staked_balance, ctx);
  
  let StakeReceipt { id, amount: _, hours_staked: _, last_update_timestamp: _ } = receipt;
  object::delete(id);
  
  (flag::new(b"staking".to_string(), ctx), staked_coin)
}

public fun unstake(
  pool: &mut StakingPool,
  receipt: StakeReceipt,
  ctx: &mut TxContext
): Coin<SUI> {
  let amount = receipt.amount;
  
  let staked_balance = balance::split(&mut pool.balance, amount);
  let staked_coin = coin::from_balance(staked_balance, ctx);
  
  let StakeReceipt { id, amount: _, hours_staked: _, last_update_timestamp: _ } = receipt;
  object::delete(id);
  
  staked_coin
}
