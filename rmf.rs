use std::env;
use std::fs;
use std::io;

pub fn run() {
    let file_path = if let Some(arg) = env::args().nth(2) {
        arg
    } 
	else {
        eprintln!("Error: No file given");
		
        std::process::exit(1);
    };

    match fs::remove_file(&file_path) {
        Ok(_) => {
            println!("Successfully removed file: '{}'", file_path);
        }
		
        Err(error) => {
            match error.kind() {
                io::ErrorKind::NotFound => {
                    eprintln!("Error: File '{}' not found", file_path);
                }
                io::ErrorKind::PermissionDenied => {
                    eprintln!("Error: Permission to remove denied for '{}'", file_path);
                }
                _ => {
                    eprintln!("Error: Can't remove '{}': {}", file_path, error);
                }
            }
			
            std::process::exit(1);
        }
    }
}
