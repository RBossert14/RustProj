use std::env;
use std::io;

pub fn run() {
    match env::current_dir() {
        Ok(path) => {
            match path.to_str() {
                Some(directory) => println!("{}", directory),
				
                None => {
                    eprintln!("Error: Unable to convert path to a valid string.");
					
                    std::process::exit(1);
                }
            }
        }
        Err(error) => {
            match error.kind() {
                io::ErrorKind::NotFound => {
                    eprintln!("Error: Current directory not found.");
                }
				
                io::ErrorKind::PermissionDenied => {
                    eprintln!("Error: Permission denied to access current directory.");
                }
				
                _ => {
                    eprintln!("Error: Unable to determine current directory: {}", error);
                }
            }
			
            std::process::exit(1);
        }
    }
}
