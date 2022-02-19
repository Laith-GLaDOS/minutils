use std::fs;
use std::path::Path;
use std::env;

fn main() {
    let mut recursive = false;
    let mut verbose = false;
    let mut suppress_errors = false;
    let mut errored = false;
    for arg in env::args().skip(1) {
        if arg.starts_with("-") {
            for element in arg.chars().skip(1) {
                if element == 'r' {
                    recursive = true;
                } else if element == 'v' {
                    verbose = true;
                } else if element == 'f' {
                    suppress_errors = true;
                } else if element == 'h' {
                    println!("rm [-rvfh] <files>");
                    return;
                } else {
                    panic!("rm: invalid option -- '{}'\nTry 'rm -h' for more information.", element);
                }
            }
        } else {
            let result = if recursive {
                fs::remove_dir_all(Path::new(&arg))
            } else {
                fs::remove_file(Path::new(&arg))
            };
            match result {
                Ok(_) => {},
                Err(e) => {
                    if e.to_string().contains("20") { // check if the error is OS Error 20 (directory is unexpectedly a file)
                        match fs::remove_file(&arg) { // try to remove the file
                            Ok(_) => {},
                            Err(_) => {
                                if !suppress_errors {
                                    eprintln!("rm: cannot remove '{}': Permission denied", arg);
                                    errored = true;
                                }
                            }
                        }
                    } else if e.to_string().contains("2") { // check if the error is OS Error 2 (file or folder not found)
                        if !suppress_errors { // if so, check if the user wants to suppress errors
                            eprintln!("rm: cannot remove '{}': No such file or directory", arg); // if not, print the error
                            errored = true;
                        }
                    }
                }
            }
            if Path::new(&arg).exists() { // check if the file was successfully removed or not
                if !suppress_errors { // if not, check if the user wants to suppress errors
                    eprintln!("rm: cannot remove '{}': Permission denied", arg); // if not, print the error
                    errored = true;
                }
            }
            if verbose && !errored { // check if verbose is true and there are no errors
                println!("removed '{}'", arg); // if so, print the removed file
            }
            errored = false;
        }
    }
}