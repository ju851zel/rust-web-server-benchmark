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

/// Wrapper for The user provided directory
type Directory = Arc<HashMap<String, Vec<u8>>>;
/// Wrapper for the fixed length byte buffer
type Buffer = [u8; 2048];


fn main() {
    println!("Starting the webserver!");

    let (ip, port, dir, threads, type_) = cli::start_cli();

    println!("Serving directory: {}", dir.cyan());

    let dir = load_provided_directory(Path::new(&dir));

    match &type_[..] {
        "threaded" => {
            println!("Server is a {} server\n Server is listening on {}:{}",
                     type_.cyan(), ip.to_string().cyan(), port.to_string().cyan());
            threaded::start_server(ip, port, threads, dir)
        }
        "event_loop" => {
            println!("Server is a {} server\n Server is listening on {}:{}",
                     type_.cyan(), ip.to_string().cyan(), port.to_string().cyan());
            event_loop::start_server(ip, port, dir)
        }
        "rouille" => {
            println!("Server is a {} server\n Server is listening on {}:{}",
                     type_.cyan(), ip.to_string().cyan(), port.to_string().cyan());
            rouille::start_server(ip, port, dir)
        }
        _ => {
            let ip_t = ip.clone();
            let port_t = port + 1;
            let dir_t = dir.clone();
            thread::spawn(move || threaded::start_server(ip_t, port_t, threads, dir_t));

            let ip_e = ip.clone();
            let port_e = port + 2;
            let dir_e = dir.clone();
            thread::spawn(move || event_loop::start_server(ip_e, port_e, dir_e));

            println!("Starting all servers\n\
                      Threaded server is listening on {ip}:{port_t}\n\
                      Event loop server is listening on {ip}:{port_e}\n\
                      Rouille server is listening on {ip}:{port}",
                     ip = ip.to_string().cyan(),
                     port_t = (port_t).to_string().cyan(),
                     port_e = (port_e).to_string().cyan(),
                     port = (port).to_string().cyan());

            rouille::start_server(ip, port, dir);
        }
    };
}

fn load_provided_directory(dir: &Path) -> Directory {
    Arc::new(file::load_directory(dir))
}

