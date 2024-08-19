use std::env;
use std::fs;



pub fn is_directory_present(id: &String) -> bool {
    let current_dir = env::current_dir().expect("Failed to get current directory");

    let dir = current_dir.join(id);

    dir.is_dir()
}

pub fn create_new_dir(id: &str) -> Result<(), std::io::Error> {
    let current_dir = env::current_dir()?;
    
    let dir_path = current_dir.join(id);
    
    fs::create_dir(&dir_path)?;
    
    println!("Directory created: {:?}", dir_path);
    Ok(())
}

use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct Vars {
    pub chromedriver: String,
    pub ffmpeg: String,
}

impl Vars {
    fn new() -> Self {
        dotenv().ok();
        let chromedriver = std::env::var("CHROMEDRIVER").expect("env info: CHROMEDRIVER must be set.");
        let ffmpeg = std::env::var("FFMPEG").expect("env info: FFMPEG must be set.");


        Self {
            chromedriver: chromedriver,
            ffmpeg: ffmpeg,
        }
    }
}
pub fn read_env() -> Vars {
    Vars::new()
}