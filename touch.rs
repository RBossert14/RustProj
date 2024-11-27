use std::env;
use std::fs;
use std::io;

pub fn run() {
    
    let file_path = if let Some(arg) = env::args().nth(2) {
        arg
    } 
	else {
        eprintln!("Error: No file specified.");
        
		std::process::exit(1);
    };

    match fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&file_path)
    {
        Ok(_) => {
            println!("File '{}' created/updated.", file_path);
        }
		
        Err(error) => {
            match error.kind() {
                io::ErrorKind::PermissionDenied => {
                    eprintln!("Error: Permission denied for '{}'", file_path);
                }
                io::ErrorKind::NotFound => {
                    eprintln!("Error: Path '{}' not found", file_path);
                }
                _ => {
                    eprintln!("Error: Could not create or update '{}': {}", file_path, error);
                }
            }
			
            std::process::exit(1);
        }
    }
}
