use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct FileData {
    file_path: String,
    file_name: String
}

fn hash_path(path: String) -> u64 {
    let mut hasher = DefaultHasher::new();
    path.hash(&mut hasher);
    hasher.finish()
}

pub fn hash_target_location(hash_map: &mut HashMap<u64, FileData>, path: String) {
    let path_content = std::fs::read_dir(path.clone()).unwrap();
    for content in path_content {
        if content.as_ref().unwrap().path().is_file() {
            let content_str = content.unwrap().path();
            let key = hash_path(content_str.to_str().unwrap().to_string());
            let file = FileData {
                file_path: content_str.to_str().unwrap().to_string(),
                file_name: content_str.to_str().unwrap()[path.len()..content_str.to_str().unwrap().to_string().len()].to_string()
            };
            hash_map.insert(key, file);
        }
        else {
            hash_target_location(hash_map, content.unwrap().path().to_str().unwrap().to_string());
        }
    }
}

pub fn find_hash_value(hash_map: &HashMap<u64, FileData>, key: u64) -> String {
    let result = hash_map.get(&key);
    match result {
        Some(value) => (&value.file_path).to_string(),
        None => "File Not Found".to_string()
    }
}

pub fn search_file_by_name(hash_map: &HashMap<u64, FileData>, file_name: &str) -> String {
    for (_, file_data) in hash_map.iter() {
        if file_data.file_name == "\\".to_string() + file_name {
            return file_data.file_path.clone();
        }
    }
    return "FILE NOT FOUND".to_string();
}