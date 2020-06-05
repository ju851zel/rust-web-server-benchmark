use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::Path;

mod cli;
mod file_util;
mod server;

fn main() {
    let (address, directory, thread_pool_size) = cli::start_cli();

    let _directory = load_directory(directory);

    server::start_server(address.as_str(), thread_pool_size as usize);
}


// todo finish loading of directory in memory and returning it
fn load_directory(path: String) {
    match file_util::get_all_files_in_dir(Path::new(path.as_str())) {
        Err(err) => { println!("{:?}", err) }
        Ok(list) => println!("{:#?}", list)
    };
}