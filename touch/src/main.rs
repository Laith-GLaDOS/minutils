use std::fs::File;
use std::path::Path;
use std::env;

fn main() {
    for arg in env::args().skip(1) {
        match File::create(&arg) {
            Ok(_) => {},
            Err(e) => {
                if e.to_string().contains("21") { // check if the error is OS Error 21
                    eprintln!("touch: cannot make folders; use mkdir instead");
                }
            }
        }
        if !Path::new(&arg).exists() { // check if the file was successfully touched or not
            eprintln!("touch: cannot touch '{}': Permission denied", arg); // if not, print error
        }
    }
}
