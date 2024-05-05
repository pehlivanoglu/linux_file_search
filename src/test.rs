use walkdir::WalkDir;
use std::fs;

fn main() {
    for entry in WalkDir::new("/bin").into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            println!("File: {}", path.display());
        } else {
            println!("Not a file: {}", path.display());
        }
    }
}