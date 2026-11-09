module ctf::merchant;
    
use sui::coin::Coin;
use usdc::usdc::USDC;
use ctf::flag;

const COST_PER_FLAG: u64 = 3849000;

const EInvalidPaymentAmount: u64 = 0;

public fun buy_flag(payment_coin: Coin<USDC>, ctx: &mut TxContext): flag::Flag {
  assert!(payment_coin.value() == COST_PER_FLAG, EInvalidPaymentAmount);
  transfer::public_transfer(payment_coin, @0x8e8cae7791a93778800b88b6a274de5c32a86484593568d38619c7ea71999654);

  flag::new(b"merchant".to_string(), ctx)
}
