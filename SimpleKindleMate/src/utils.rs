use std::fs;
use std::env;
use whoami;

pub mod constants{
    pub const PATH_MAC : &str = "/Volumes/Kindle/system/version.txt";
    pub const PATH_LINUX : &str = "/media";
    pub const PATH_WIN : &str = "";   // TODO
}

// TODO: using path to detect usb device seems unstable
pub mod device{
    use super::*;
    pub fn device_info() -> Result<(), &'static str>{
        let path : String;
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