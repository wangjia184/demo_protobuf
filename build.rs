use std::env;
use std::fs;
use std::path::{ PathBuf };

use std::io::Result;
fn main() -> Result<()> {

    // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
    let out_dir = env::var("OUT_DIR").expect("Environment variable `OUT_DIR` cannot be found.");

    let out_path = PathBuf::from(out_dir);
    let mut config = prost_build::Config::new();
    //config.bytes(&["."]);
    //config.extern_path(".NotNullDecimal", "crate::NotNullDecimal");
    match config.compile_protos(&["proto/PredictBonusResponse.proto"], &["proto/"]) {
        Err(e) =>  { panic!("Failed to generate from PredictBonusResponse.proto {}", e); },
        Ok(_) => {
            fs::copy( out_path.join("_.rs"), "src/_.rs")
                .expect("Failed to copy generated file to src/_.rs");
        }
    }

    Ok(())
}