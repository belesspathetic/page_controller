use std::env;
use std::env::current_dir;
use std::fs::{self, remove_file, File};
use std::io::{BufRead, BufReader, Error, Read};
use std::io::Write;


pub async fn ensure_directory_structure(id: &String) -> Result<(), Box<dyn std::error::Error>> {
    if !is_directory_present(id).await? {
        create_new_dir(id).await?;
    }

    if !is_status_file_exist(id).await {
        create_status_file(id);
    }
    Ok(())
}


pub async fn is_directory_present(id: &String) -> Result<bool, Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;

    let dir = current_dir.join(id);


    Ok(dir.is_dir())
}

pub async fn create_new_dir(id: &str) -> Result<(), std::io::Error> {
    let current_dir = env::current_dir()?;
    
    let dir_path = current_dir.join(id);
    
    fs::create_dir(&dir_path)?;
    
    println!("Directory created: {:?}", dir_path);
    Ok(())
}

pub async fn is_status_file_exist(id: &String) -> bool {
    current_dir().unwrap().join(id).join("status.txt").is_file()
}

pub fn create_status_file(id: &String) {
    let path = current_dir().unwrap().join(id).join("status.txt");
    let _ = File::create(path);
}

pub fn update_status_file(id: &String, status: Status) {
    let file_path = current_dir().unwrap().join(id).join("status.txt");
    let mut lines = Vec::new();

    let file = File::open(&file_path).unwrap();
    let reader = BufReader::new(file);
    lines.extend(reader.lines().filter_map(Result::ok));

    if !lines.is_empty() {
        lines[0] = status.to_string();
    } else {
        lines.push(status.to_string());
    }

    let mut file = File::create(&file_path).unwrap();
    for line in lines {
        writeln!(file, "{}", line).unwrap();
    }

}

pub fn read_status(path: String) -> Result<String, Error> {
    let mut file = File::open(path)?;
    
    // Создаем строку для хранения содержимого
    let mut status = String::new();
    
    // Читаем содержимое файла в строку
    let _ = file.read_to_string(&mut status);

    Ok(status.trim().to_string())
}

pub fn remove_status_file(id: &String) {
    let file_path = current_dir().unwrap().join(id).join("status.txt");
    remove_file(file_path).unwrap();
}

use dotenv::dotenv;
use shared::models::Status;

#[derive(Debug, Clone)]
pub struct Vars {
    //pub chromedriver: String,
    pub ffmpeg: String,
}

impl Vars {
    fn new() -> Self {
        dotenv().ok();
        //let chromedriver = std::env::var("CHROMEDRIVER").expect("env info: CHROMEDRIVER must be set.");
        let ffmpeg = std::env::var("FFMPEG").expect("env info: FFMPEG must be set.");


        Self {
            //chromedriver: chromedriver,
            ffmpeg: ffmpeg,
        }
    }
}
pub fn read_env() -> Vars {
    Vars::new()
}