use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher}; 
use std::fs;

pub fn test_hash_key_collisions(path: String) {
    let mut collision_counter: u64 = 0;
    let mut counter: u64 = 0;
    let mut key_vector: Vec<u64> = Vec::new();
    println!("STARTING COLLISION DETECTION");
    fs_walker(path, &mut key_vector, &mut collision_counter, &mut counter);
    println!("ENDING COLLISION DETECTION");
    println!("Counter : {}\n Collisions : {}", counter, collision_counter);
}

fn fs_walker(path: String, key_vec: &mut Vec<u64>, collision_counter: &mut u64, counter: &mut u64) {
    let len_path = path.len() + 1;
    let folder_content = match fs::read_dir(&path) {
        Ok(dir) => dir,
        Err(e) => {
            println!("Got Error for {:?} e {:?}", path, e);
            return
        }
    };
    for content in folder_content {
        let unwrapped_content: std::path::PathBuf = content.unwrap().path();
        let unwrapped_content_str: &str = unwrapped_content.to_str().unwrap();
        if unwrapped_content.is_dir() {
            fs_walker(unwrapped_content_str.to_string(), key_vec, collision_counter, counter);
        }
        else if unwrapped_content.is_file() {
            *counter = *counter + 1;
            //println!("checking {:?}", unwrapped_content_str);
            let file_name = unwrapped_content.to_str().unwrap()[len_path..].to_string();
            let key = hash_path(&file_name);
            match key_vec.binary_search(&key) {
                Ok(v1) => {
                    println!("COLLISION DETECTED! for file {}", file_name);
                    *collision_counter = *collision_counter + 1;
                },
                Err(e) => {
                    key_vec.push(key);
                }
            }
        }
    }
}


pub fn hash_path(path: &String) -> u64 {
    let mut hasher = DefaultHasher::new();
    path.hash(&mut hasher);
    hasher.finish()
}

pub fn hash_map_of_target_location(hashmap: &mut HashMap<u64, Vec<String>>, path: String) {
    let folder_content = std::fs::read_dir(&path).unwrap();
    let len_path = path.len() + 1;
    for content in folder_content {
        let unwrapped_content = content.unwrap().path();
        let unwrapped_content_str = unwrapped_content.to_str().unwrap().to_string();
        if unwrapped_content.is_dir() {
            hash_map_of_target_location(hashmap, unwrapped_content_str);
        }
        else {
            let file_name = unwrapped_content.to_str().unwrap()[len_path..].to_string();
            match hashmap.entry(hash_path(&file_name)) {
                // If the key exists, push the value to the vector
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    entry.get_mut().push(unwrapped_content_str);
                }
                // If the key does not exist, insert a new vector with the value
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(vec![unwrapped_content_str]);
                }
            }
        }
    }
}

pub fn hash_map_get_path(hashmap: &HashMap<u64, Vec<String>>, key: u64) {
    let value_ref = hashmap.get(&key).unwrap();
    println!("Found in {:?}", value_ref);
}