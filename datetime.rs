use chrono::Local;

pub fn run() {
    let cur_time = Local::now();
    println!("{}", cur_time.format("%a %b %e %T %Y"));
}
