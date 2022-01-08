use std::fs;
use std::env;
use whoami;

// TODO: how to detect usb munt path?
pub mod constants{
    pub const PATH_MAC : &str = "/Volumes/Kindle/system/version.txt";
    pub const PATH_LINUX : &str = "/media";
    pub const PATH_WIN : &str = "E:\\";
}

// kindle's vid=0x1949, pid=0x0324
pub mod device{
    use super::*;
    use rusb;

    pub fn is_connected() ->bool{
        for device in rusb::devices().unwrap().iter(){
            let device_desc = device.device_descriptor().unwrap();
            if device_desc.vendor_id() == 0x1949 &&
               device_desc.product_id() == 0x0324{
                   return true;
               }
        }
        false
    }

    pub fn device_info() -> Result<(), &'static str>{
        let mut path : String = String::from("");
        match env::consts::OS{
            "macos" => {
                path = constants::PATH_MAC.to_owned();
            }
            "linux" =>{
                let user_name = &whoami::username()[..];
                path = format!("{}/{}/Kindle/system/version.txt", constants::PATH_LINUX, user_name);
            }
            "windows"=>{
                path = constants::PATH_WIN.to_owned();
            }
            _ =>{
                return Err("Unsupported platform!");
            }
        }
        if let Ok(_file) = fs::File::open(&path){
            if let Ok(content) = fs::read_to_string(&path) {
                println!("device: {}", content);
            } else{
                return Err("Cannot get device information. Read file error.");
            }
        } else{
            return Err("No device detected. Is it connected?");
        }
        Ok(())
    }
}