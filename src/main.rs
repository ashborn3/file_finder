mod filewalker;

fn main() {
    let init_path = "D:/Games";

    println!("preformatting : {}\npostformatting : {:?}", init_path, format_string_for_fs(init_path));

    filewalker::read_folder(&format_string_for_fs(init_path));
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