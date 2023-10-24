use std::fs;

pub fn recursive_read_folder(path: &str) {
    let folder_contents: fs::ReadDir = fs::read_dir(path).unwrap();
    for content in folder_contents {
        let unwrapped_content: std::path::PathBuf = content.unwrap().path();
        let unwrapped_content_str: &str = unwrapped_content.to_str().unwrap();
        if unwrapped_content.is_dir() {
            println!("Moving into {:?}", unwrapped_content_str.replace(path, "-"));
            recursive_read_folder(unwrapped_content_str);
        }
        else if unwrapped_content.is_file() {
            println!("{:?}", unwrapped_content_str.replace(path, "-"));
        }
    }
    println!("Moving out {:?}\n", path);
}