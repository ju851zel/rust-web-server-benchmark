use std::sync::Arc;
use std::collections::HashMap;
use std::thread;
use std::path::Path;

mod threaded;
mod rouille;
mod event_loop;
mod request;
mod response;
mod file;
mod cli;

use colored::Colorize;
use std::thread::JoinHandle;

type Directory = Arc<HashMap<String, Vec<u8>>>;

fn main() {
    println!("Starting the webserver!");

    let (ip, port, dir, threads, type_) = cli::start_cli();

    println!("Server is a {} server", type_.cyan());
    println!("Server listening on {}:{}", ip.to_string().cyan(), port.to_string().cyan());
    println!("Serving directory: {}", dir.cyan());

    let dir = load_provided_directory(Path::new(&dir));

    match &type_[..] {
        "threaded" => threaded::start_server(ip, port, threads, dir),
        "event_loop" => event_loop::start_server(ip, port, dir),
        "rouille" => rouille::start_server(ip, port, dir),
        _ => {
            let ip_t = ip.clone();
            let dir_t = dir.clone();
            thread::spawn(move || threaded::start_server(ip_t, port, threads, dir_t));

            let ip_e = ip.clone();
            let dir_e = dir.clone();
            thread::spawn(move || event_loop::start_server(ip_e, port, dir_e));

            rouille::start_server(ip, port, dir);
        }
    };
}

fn load_provided_directory(dir: &Path) -> Directory {
    Arc::new(file::load_directory(dir))
}
