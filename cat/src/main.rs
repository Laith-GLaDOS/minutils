use std::fs;
use std::env;

fn main() {
    for arg in env::args().skip(1) {
        let contents = fs::read_to_string(&arg); // read file contents
        match contents {
            Ok(contents) => print!("{}", contents), // print contents if read was successful
            Err(e) => { // if read was unsuccessful
                if e.kind() == std::io::ErrorKind::NotFound { // if file does not exist
                    println!("cat: {}: No such file or directory", &arg); // print no file message
                } else { // else its a permission issue
                    println!("cat: {}: Permission denied", &arg); // print permission denied message
                }
            }
        }
    }
}
