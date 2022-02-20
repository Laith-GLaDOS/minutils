use std::env;

fn main() {
    loop {
        for arg in env::args().skip(1) {
            print!("{} ", arg);
        }
        println!();
    }
}