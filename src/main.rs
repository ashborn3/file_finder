use std::collections::HashMap;
use std::time::Instant;
use std::env;

mod hashmap;
mod cachehash;

fn main() {
    let search_string = match env::args().nth(1) {
        Some(value) => value,
        None => panic!("No Argument Given")
    };

    let init_path = "C:/";

    let formatted_path = format_string_for_fs(init_path);

    let now_read_hash_from_cache = Instant::now();
    
    // 2.34% collision percentage benchmarked on C drive
    // hashmap::test_hash_key_collisions(formatted_path);

    let mut fs_hash_map: HashMap<u64, Vec<String>> = HashMap::new();

    fs_hash_map = cachehash::get_hash_from_cache();

    let time_taken_to_read_hash_from_cache = now_read_hash_from_cache.elapsed();

    println!("Reading HashMap from cache.bin took {:?}", time_taken_to_read_hash_from_cache);

    let now1 = Instant::now();
    hashmap::hash_map_get_path(&fs_hash_map, hashmap::hash_path(&search_string));

    let now2 = now1.elapsed();

    println!("Took {:?} to Search for {}", now2, search_string);

    // while true {
    //     print!("Enter your name : ");
        
    //     std::io::stdin().read_line(&mut search_string).unwrap();

    //     let now1 = Instant::now();
    //     hashmap::hash_map_get_path(&fs_hash_map, hashmap::hash_path(&search_string));

    //     let now2 = now1.elapsed();

    //     println!("Took {:?} to Search for {}", now2, search_string);
    // }
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