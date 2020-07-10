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
use crate::file::load_dynamic_resources;

/// Wrapper for all statically server files. The directory provided by the user, as well as from the resources directory
type StaticFiles = Arc<HashMap<String, Vec<u8>>>;
type DynamicFiles = Arc<HashMap<String, String>>;
/// Wrapper for the fixed length byte buffer
type Buffer = [u8; 2048];

/// Starts all the webservers depending on the users input
fn main() {
    let (ip, port, dir, threads, type_) = cli::start_cli();

    println!("Serving directory: {}", dir.cyan());

    let static_files = match load_static_files(Path::new(&dir)) {
        Ok(static_files) => static_files,
        Err(error) => {
            println!("{}",error);
            return;
        }
    };

    println!("Successfully read dir in memory: {:#?}", dir.keys());

    println!("Starting the webserver/s!");

    let dynamic_files = Arc::new(load_dynamic_resources().unwrap());

    let server_files = ServerFiles { static_files, dynamic_files };

    match &type_[..] {
        "threaded" => {
            println!("Server is a {} server\n Server is listening on {}:{}",
                     type_.cyan(), ip.to_string().cyan(), port.to_string().cyan());
        }
        // "event_loop" => {
        //     println!("Server is a {} server\n Server is listening on {}:{}",
        //              type_.cyan(), ip.to_string().cyan(), port.to_string().cyan());
        //     event_loop::start_server(ip, port, dir)
        // }
        "rouille" => {
            println!("Server is a {} server\n Server is listening on {}:{}",
                     type_.cyan(), ip.to_string().cyan(), port.to_string().cyan());
            rouille::start_server(ip, port, dir);//todo
        }
        _ => {
            let ip_t = ip.clone();
            let port_t = port + 1;
            let dir_t = dir.clone();
            thread::spawn(move || threaded::start_server(ip_t, port_t, threads, dir_t, dynamic_files));

            let ip_e = ip.clone();
            let port_e = port + 2;
            let dir_e = dir.clone();
            // thread::spawn(move || event_loop::start_server(ip_e, port_e, dir_e));

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

fn load_static_files(dir: &Path) -> Result<StaticFiles, String> {

    let provided_directory = file::load_directory(Path::new(&dir))?;
    let static_respources = file::load_directory(Path::new(&dir))?;

    let dir = match file::load_directory(Path::new(&dir)) {
        Ok(dir) => Arc::new(dir),
        Err(error) => {
            println!("{}",error);
            return;
        }
    };

    let dir2 = match file::load_directory(Path::new(&dir)) {
        Ok(dir) => Arc::new(dir),
        Err(error) => {
            println!("{}",error);
            return;
        }
    };

    let static_files = provided_directory.into_iter().chain(static_respources).collect();



    Ok(Arc::new(static_files))
}
