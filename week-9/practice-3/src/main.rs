use std::fs;

fn main() {
    fs::remove_file("leo.txt").expect("could not remove the file");
    println!("file was removed");
}