use std::process;

mod utils;
use utils::device;

fn main() {
    if let Err(err) = device::device_info(){
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
