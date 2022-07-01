#![macro_use]
extern crate  fpdec_macros;
extern crate base64;
use fpdec::Decimal;

use std::io::Cursor;
use prost::{ Message };


mod dto;



fn main() {

    // copied from C# result
    let buf = base64::decode("CAIg78zT2AE6Egia1prSja7F3yUQ4pab4QoYIkISCLCh24iZm4uqFRC5zeHnARga").unwrap();
 
    let resp = dto::PredictBonusResponse::decode(&mut Cursor::new(buf)).unwrap();

    let amount = resp.remaining_total_standard_bonus_amount.unwrap();
    println!("{}", &amount);

    // convert to a 3rd-party fixed point 128bits decimal
    let amount : Decimal = amount.into();
    println!("{}", amount);
   
}




