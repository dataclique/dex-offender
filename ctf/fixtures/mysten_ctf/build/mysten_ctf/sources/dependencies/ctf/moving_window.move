module ctf::moving_window;

use sui::clock::Clock;
use ctf::flag;

const EWindowClosed: u64 = 0;

public fun extract_flag(clock: &Clock, ctx: &mut TxContext): flag::Flag {
  let timestamp_seconds = clock.timestamp_ms() / 1000;
  let time_in_hour = timestamp_seconds % 3600;
  
  assert!(
    (time_in_hour >= 0 && time_in_hour < 300) ||
    (time_in_hour >= 1800 && time_in_hour < 2100),
    EWindowClosed
  );

  flag::new(b"moving_window".to_string(), ctx)
}
