use std::fs::File;
use std::io::{Write,Read,Seek,SeekFrom};

pub fn init_disk(path: &str, size: usize) {
    let mut file = File::create(path).expect("Failed to create disk");
    file.write_all(&vec![0u8; size]).expect("Failed to initialize disk");
}

pub fn load_disk(path:&str)->Vec<u8>{
    let mut file = File::open(path).expect("failed to open the path");
    let mut buffer = Vec::new();
    file.seek(SeekFrom::Start(0)).unwrap();
    file.read_to_end(&mut buffer).expect("failed to read the file");
    buffer 
}