use std::{env, fs, io};

fn main() {

    let test_dir = "/Users/zhangyesheng/Workspace/rust/rust_practice";

    fn start (target_dir: String) {
        let entries = fs::read_dir(target_dir).unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path().as_path().to_str().unwrap().to_string();
            if entry.file_type().unwrap().is_dir() {
                println!("dir: {}", path);
                start(path);
                return
            }
            println!("path: {}", path);
        }
    }

    start(String::from(test_dir));
}
