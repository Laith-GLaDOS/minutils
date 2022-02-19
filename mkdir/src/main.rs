use std::fs;
use std::io::ErrorKind;
use std::env;

fn main() {
    let mut verbose = false;
    for arg in env::args().skip(1) {
        if arg.starts_with("-") {
            for element in arg.chars().skip(1) {
                if element == 'h' {
                    println!("mkdir [-v] <dirs>");
                    return;
                } else if element == 'v' {
                    verbose = true;
                }
            }
        }
    }
    for arg in env::args().skip(2) {
        match fs::create_dir(&arg) {
            Ok(_) => {
                if verbose {
                    println!("mkdir: created directory '{}'", &arg);
                }
            },
            Err(e) => {
                if e.kind() == ErrorKind::AlreadyExists {
                    println!("mkdir: cannot create directory '{}': File exists", &arg);
                } else if e.kind() == ErrorKind::PermissionDenied {
                    eprintln!("mkdir: cannot create directory '{}': Permission denied", arg);
                } else {
                    eprintln!("mkdir: cannot create directory '{}': No space left on device", arg);
                }
            }
        }
    }
}
