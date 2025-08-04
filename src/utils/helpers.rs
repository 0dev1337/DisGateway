use std::fs::File;
use std::io::Write;

pub fn save_to_file(server_id : &str){
    let mut file = File::create("servers.txt").unwrap();
    file.write_all(format!("{}\n",server_id).as_bytes()).unwrap();
    file.flush().unwrap();
}