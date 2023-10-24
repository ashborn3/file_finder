use std::collections::HashMap;
use std::time::Instant;

mod filewalker;
mod hashmap;

fn main() {
    let init_path = "D:/Games";

    let formatted_path = format_string_for_fs(init_path);

    let now1 = Instant::now();

    let mut fs_hash_map: HashMap<u64, hashmap::FileData> = HashMap::new();

    hashmap::hash_target_location(&mut fs_hash_map, formatted_path);

    let elapsed = now1.elapsed();

    println!("Created Hash Map! took : {:?}", elapsed);

    //println!("\n\n{:?}\n\n", fs_hash_map);

    let now2 = Instant::now();

    println!("Searching for baselib.dll\n");

    println!("File Found at {}", hashmap::search_file_by_name(&fs_hash_map, "baselib.dll"));

    let elapse = now2.elapsed();

    println!("Elapsed Time for Searching : {:?}", elapse);

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