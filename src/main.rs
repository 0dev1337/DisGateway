mod api;
mod models;
mod utils;


fn main() {
    let token = "MTQwMDc4NjE5NjEzMjUzMjMzNw.GZH_qB.URZmWwlYB0J1Z1vXwR1yqA5-li7T5Z6QD7-6EE";
    for i in 33..100 {
        if let Err(e) = api::guilds::create_server(i,token) {
            eprintln!("Error creating server #{}: {}", i, e);
        }
    }
}
