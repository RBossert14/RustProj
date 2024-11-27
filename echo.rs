use std::env;

pub fn run() {
    let mut is_first_arg = true;

    for arg in env::args().skip(2) {
        if !is_first_arg {
            print!(" ");
        }

        print!("{}", arg);
        is_first_arg = false;
    }

    println!();
}