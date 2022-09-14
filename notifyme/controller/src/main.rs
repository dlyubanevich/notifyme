use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("Hello, world!");
    let sys_time = SystemTime::now();
    let d = sys_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
}
