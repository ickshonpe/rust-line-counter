use std::env;
use std::fs;
use std::fs::File;
use std::fs::ReadDir;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;

fn count_lines(file_name: PathBuf) -> usize {  
    let f = File::open(file_name).unwrap();
    let f = BufReader::new(f);    
    //f.lines().count() 
    let mut count = 0;
    for line in f.lines() {
        if let Ok(line) = line {
            if line.len() != 0 {
                count += 1;
            }
        }
    }
    count
}

fn recursively_count_rust_lines(path: ReadDir) -> usize {
    let mut count = 0;
    for r in path {
        if let Ok(dir_entry) = r {
            if dir_entry.file_type().unwrap().is_dir() {
                count += recursively_count_rust_lines(dir_entry.path().read_dir().unwrap());
            }
            if dir_entry.file_type().unwrap().is_file() {
                if let Some(extension) = dir_entry.path().extension() {
                    if extension == "rs" {
                        count += count_lines(dir_entry.path());
                    }
                }
            }
        }
    }
    count
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path_name = if args.len() > 1 { &args[1] } else { "." };
    if let Ok(count) = fs::read_dir(path_name).map(recursively_count_rust_lines) {
        println!("Lines of Rust in directory {:?} and its subdirectories = {}", path_name, count);
    }
}
