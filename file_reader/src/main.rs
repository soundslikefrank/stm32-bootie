use itertools::join;
use std::fs;

fn main() {
    match fs::read("../../../blblinky/target/thumbv7em-none-eabihf/release/blblinky.bin") {
        Ok(res) => {
            let len = res.len().to_string();
            let x = [
                "pub const BYTES: [u8; ",
                &len,
                "] = [",
                &join(res.iter(), ", "),
                "];",
            ]
            .join("");
            print!("{}", x);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
