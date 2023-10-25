use std::collections::HashMap;
use std::time::Instant;

mod hashmap;

fn main() {
    let init_path = "D:/Games";

    let formatted_path = format_string_for_fs(init_path);

    let file: String = "baselib.dll".to_string();

    let now1 = Instant::now();

    let mut fs_hash_map: HashMap<u64, Vec<String>> = HashMap::new();

    hashmap::hash_map_of_target_location(&mut fs_hash_map, formatted_path);

    let now1a = now1.elapsed();

    println!("Took {:?} to create the hash map", now1a);

    let now2 = Instant::now();

    hashmap::hash_map_get_path(&fs_hash_map, hashmap::hash_path(file));

    let now2a = now2.elapsed();

    println!("Took {:?} to find the file", now2a);
}

fn format_string_for_fs(str: &str) -> String {
    let mut final_str = String::new();
    for ch in str.chars() {
        if ch == '/' {
            final_str += "\\";
        }
        else {
            final_str.push(ch);
        }
    }
    final_str
}