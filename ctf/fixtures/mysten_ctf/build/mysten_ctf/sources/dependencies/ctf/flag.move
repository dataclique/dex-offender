module ctf::flag;

use std::string::String;
use sui::display;
use sui::package;

public struct Flag has key, store {
  id: UID,
  source: String
}

public struct FLAG has drop {}

fun init(otw: FLAG, ctx: &mut TxContext) {
  // Set up Display for Ticket NFT
  let keys = vector[
    b"name".to_string(),
    b"image_url".to_string(),
    b"project_url".to_string(),
  ];

  let values = vector[
    // Ticket name: "Username's Ticket #123"
    b"{source} flag".to_string(),
    // image url
    b"https://media.tenor.com/t3eKwU-odDgAAAAM/sui-siu.gif".to_string(),
    // Project URL
    b"https://github.com/MystenLabs/CTF".to_string(),
  ];

  let publisher = package::claim(otw, ctx);
    
  let mut display = display::new_with_fields<Flag>(&publisher, keys, values, ctx);

  display::update_version(&mut display);

  transfer::public_transfer(publisher, ctx.sender());
  transfer::public_transfer(display, ctx.sender());
}

public(package) fun new(source: String, ctx: &mut TxContext): Flag {
  Flag {
    id: object::new(ctx),
    source
  }
}

public fun delete(flag: Flag) {
  let Flag { id, source: _ } = flag;
  object::delete(id);
}
