use std::env;
use std::thread;

use crate::client::start_client;
use crate::server::start_server;
use crate::utils::sleep;

mod client;
mod constants;
mod server;
mod utils;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let name = args[2].clone();
    if &args[1] == "host" {
        let handle = thread::spawn(move || start_server());
        sleep();
        start_client(name);
        handle.join().unwrap();
    } else {
        start_client(name);
    }
}
