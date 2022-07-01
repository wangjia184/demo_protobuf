#![macro_use]
extern crate  fpdec_macros;
extern crate base64;
use fpdec::Decimal;

use std::io::Cursor;
use prost::{ Message };


mod dto;



fn main() {

    // copied from C# result
    let buf = base64::decode("CAIg78zT2AE6Egia1prSja7F3yUQ4pab4QoYIkITCP///////////wEQ/////w8YAQ==").unwrap();
 
    let resp = dto::PredictBonusResponse::decode(&mut Cursor::new(buf)).unwrap();

    println!("TotalLockedAmount = {}", &resp.total_locked_amount.unwrap());

    let amount : Decimal = resp.remaining_total_standard_bonus_amount.unwrap().into();
    println!("RemainingTotalStandardBonusAmount = {}", amount);
}




