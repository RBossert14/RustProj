use std::env;
use std::fs;
use std::io::{self, Read};

pub fn run() {
    let path = if let Some(arg) = env::args().nth(2) {
        arg
    } 
	else {
        eprintln!("Error: No file specified.");
		
        std::process::exit(1);
    };

    match fs::File::open(&path) {
        Ok(mut file) => {
            let mut file_text = String::new();
			
            if let Err(error) = file.read_to_string(&mut file_text) {
                eprintln!("Error reading file '{}': {}", path, error);
				
                std::process::exit(1);
            }
            print!("{}", file_text);
        }
		
        Err(error) => {
            match error.kind() {
                io::ErrorKind::NotFound => {
                    eprintln!("Error: File '{}' not found", path);
                }
                io::ErrorKind::PermissionDenied => {
                    eprintln!("Error: Permission denied for '{}'", path);
                }
                _ => {
                    eprintln!("Error: Could not open file '{}': {}", path, error);
                }
            }
			
            std::process::exit(1);
        }
    }
}
