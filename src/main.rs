use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        // bytes() needs std::io::Read
        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    println!("{:?} \r", b);  // \r The carriage return moves the cursor back to the beginning of the current line
                } else {
                    println!("{:?} ({})\r", b, c);
                }
                if b == to_ctrl_byte('c') {
                    break;
                }
            }
            Err(err) => die(err),
        }
        
    }
}
