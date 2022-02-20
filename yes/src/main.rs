use std::env;

fn main() {
    let mut two_args = false;
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        two_args = true;
    }
    if two_args {
        loop {
            for arg in env::args().skip(1) {
                print!("{} ", arg);
            }
            println!();
        }
    }
    else {
        loop {
            println!("y");
        }
    }
}