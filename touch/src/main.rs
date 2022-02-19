use std::fs::File;
use std::env;

fn main() {
    for arg in env::args().skip(1) {
        match File::create(&arg) {
            Ok(_) => {},
            Err(e) => {
                if e.to_string().contains("21") { // check if the error is OS Error 21 (is a directory)
                    eprintln!("touch: cannot make folders; use mkdir instead");
                } else if e.kind() == std::io::ErrorKind::PermissionDenied {
                    eprintln!("touch: cannot create file '{}': Permission denied", &arg);
                } else {
                    eprintln!("touch: cannot create file '{}': No space left on device", &arg);
                }
            }
        }
    }
}
