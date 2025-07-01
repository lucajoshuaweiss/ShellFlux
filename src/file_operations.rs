use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

pub fn load_scripts(scripts_dir: &str) -> HashMap<String, String> {
    let mut scripts_map = HashMap::new();
    if let Ok(entries) = fs::read_dir(scripts_dir) {
        for entry in entries.filter_map(Result::ok) {
            if let Some(file_name) = entry.path().file_stem().and_then(|s| s.to_str()) {
                if entry.path().extension().and_then(|s| s.to_str()) == Some("sh") {
                    let mut content = String::new();
                    if let Ok(mut file) = File::open(entry.path()) {
                        file.read_to_string(&mut content).ok();
                    }
                    scripts_map.insert(file_name.to_string(), content);
                }
            }
        }
    }
    return scripts_map;
}

pub fn save_script_to_file(scripts_dir: &str, title: &str, script: &str) {
    if title.is_empty() {
        return;
    }

    let path = Path::new(scripts_dir).join(format!("{title}.sh"));
    println!("Saving script to {:?}", path.display());

    if let Ok(mut file) = File::create(&path) {
        if let Err(e) = file.write_all(script.as_bytes()) {
            eprintln!("Failed to save script: {}", e);
        }
    } else {
        eprintln!("Failed to create file at {:?}", path);
    }
}

pub fn delete_script_from_file(scripts_dir: &str, title: &str) {
    let path = Path::new(scripts_dir).join(format!("{title}.sh"));
    if path.exists() {
        if let Err(e) = fs::remove_file(&path) {
            eprintln!("Failed to delete script: {}", e);
        } else {
            println!("Script '{}' deleted successfully.", title);
        }
    } else {
        println!("Script '{}' does not exist.", title);
    }
}

pub fn ensure_scripts_directory(scripts_dir: &str) {
    if !Path::new(scripts_dir).exists() {
        if let Err(e) = fs::create_dir_all(scripts_dir) {
            eprintln!("Failed to create scripts directory: {}", e);
        }
    }
}
