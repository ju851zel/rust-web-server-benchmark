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
use crate::file::{load_dynamic_files, load_static_files};

/// Wrapper for all static server files. As in the directory provided by the user, as well as from the resources directory
type StaticFiles = Arc<HashMap<String, Vec<u8>>>;
/// Wrapper for all dynamic server files from the resources directory
type DynamicFiles = Arc<HashMap<String, String>>;
/// Wrapper for the fixed length byte buffer
type Buffer = [u8; 2048];

/// Starts all the webservers depending on the users input
fn main() {
    let (ip, port, dir, threads, type_) = cli::start_cli();

    println!("Serving directory: {}", dir.cyan());

    let static_files = match load_static_files(Path::new(&dir)) {
        Ok(static_files) => Arc::new(static_files),
        Err(error) => {
            println!("{}",error);
            return;
        }
    };

    let dynamic_files = match load_dynamic_files() {
        Ok(dynamic_files) => Arc::new(dynamic_files),
        Err(error) => {
            println!("{}",error);
            return;
        }
    };

    println!("Successfully read dir in memory: {:#?}", &static_files.keys());
    println!("Starting the webserver/s!");

    match &type_[..] {
        "threaded" => {
            println!("Server is a {} server\nServer is listening on {}:{}",
                     type_.cyan(), ip.to_string().cyan(), port.to_string().cyan());
            threaded::start_server(ip, port, threads, static_files, dynamic_files);
        }
        "event_loop" => {
            println!("Server is a {} server\n Server is listening on {}:{}",
                     type_.cyan(), ip.to_string().cyan(), port.to_string().cyan());
            event_loop::start_server(ip, port, static_files)
        }
        "rouille" => {
            println!("Server is a {} server\n Server is listening on {}:{}",
                     type_.cyan(), ip.to_string().cyan(), port.to_string().cyan());
            rouille::start_server(ip, port, static_files);//todo
        }
        _ => {
            let ip_t = ip.clone();
            let port_t = port + 1;
            let dir_t = static_files.clone();
            thread::spawn(move || threaded::start_server(ip_t, port_t, threads, dir_t, dynamic_files));

            let ip_e = ip.clone();
            let port_e = port + 2;
            let dir_e = static_files.clone();
            thread::spawn(move || event_loop::start_server(ip_e, port_e, dir_e));

            println!("Starting all servers\n\
                      Threaded server is listening on {ip}:{port_t}\n\
                      Event loop server is listening on {ip}:{port_e}\n\
                      Rouille server is listening on {ip}:{port}",
                     ip = ip.to_string().cyan(),
                     port_t = (port_t).to_string().cyan(),
                     port_e = (port_e).to_string().cyan(),
                     port = (port).to_string().cyan());

            rouille::start_server(ip, port, static_files);
        }
    };
}
