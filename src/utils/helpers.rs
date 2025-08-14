use std::fs::{OpenOptions};
use std::io::{Write};
use std::fs::File;
use std::io::Read;
pub fn save_to_file(server_id: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("servers.txt")
        .unwrap();

    file.write_all(format!("{}\n", server_id).as_bytes()).unwrap();
    file.flush().unwrap();
}

pub fn read_from_file(file_name : &str)-> Vec<String>{
    let mut data_file = File::open(file_name).unwrap();
    let mut data = String::new();
    data_file.read_to_string(&mut data).unwrap();
    data.split("\n");
    data.lines().map(|line| line.to_string()).collect()

}