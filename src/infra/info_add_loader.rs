use std::fs::File;
use std::io::{Write, Read};
use std::collections::HashMap;
use chrono::NaiveDateTime;
use crate::domain::info_add::info_add::InfoAdd;
use std::path::PathBuf;

pub fn save_info_add(folder: PathBuf, data: &InfoAdd) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = folder.join("tally_data.json");
    let json = serde_json::to_string_pretty(data)?;
    
    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())?;
    
    Ok(())
}

pub fn load_info_add(folder:PathBuf) -> Result<InfoAdd, Box<dyn std::error::Error>> {
    let mut file = File::open(folder)?;
    let mut contents = String::new();
    
    file.read_to_string(&mut contents)?;
    
    let data: InfoAdd = serde_json::from_str(&contents)?;
    
    Ok(data)
}

