mod server;
mod client;
mod packets;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = args.get(1);

    match arg {
        Some(arg) => {
            if arg == "--server" {
                println!("Starting server...");
                server::server::start_server();
            } else if arg == "--client" {
                println!("Starting client...");
                client::client::start_client();
            } else {
                println!("Invalid argument {}. ", arg);
            }
        },
        None => {
            println!("No arguments provided. Starting client!");
            client::client::start_client();
        }
    }
}
