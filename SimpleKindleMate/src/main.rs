use std::process;

mod utils;
use utils::device;

fn main() {

    if device::is_connected(){
        println!("Kindle detected");
    } else{
        eprintln!("Kindle not connected");
        process::exit(1);
    }
}
