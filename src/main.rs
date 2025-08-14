mod api;
mod models;
mod utils;

fn main() {
    let token = "";
    // for i in 0..10 {
    //     if let Err(e) = api::discord::guilds::create_server(i,token) {
    //         eprintln!("Error creating server #{}: {}", i, e);
    //     }
    // }
    // return;


    let data = utils::helpers::read_from_file("servers.txt");

    for i in &data{
        if let Err(e) =api::discord::guilds::delete_server(i, token){
            eprintln!("Error deleting server #{}: {}", i, e);
        }
    }


}