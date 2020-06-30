use std::sync::Arc;
use std::collections::HashMap;
use std::thread;
use std::path::Path;

mod threaded;
mod rouille;
mod nonblocking;
mod request;
mod response;
mod file;
mod cli;

use colored::Colorize;


fn main() {
    println!("{}", "Starting the webserver!".to_string().cyan());

    let (ip, port, directory, threads, type_) = cli::start_cli();

    println!("Server listening on {}:{} with {} threads", ip, port, threads);
    println!("Serving directory: {}", directory);


    let files = load_provided_directory(Path::new(directory.as_str()));


    let my_files = files.clone();
    let rouille_files = files.clone();
    let non_blocking_files = files.clone();
    let my_ip = ip.clone();
    let rouille_ip = ip.clone();
    let non_blocking_ip = ip.clone();

    let my = thread::spawn(move || {
        threaded::start_server(my_ip, port, threads, my_files);
    });

    let rouille = thread::spawn(move || {
        rouille::start_server(rouille_ip, port + 2, rouille_files);
    });

    let non_blocking = thread::spawn(move || {
        nonblocking::start_server(non_blocking_ip, port + 1, non_blocking_files);
    });

    my.join().unwrap();
    non_blocking.join().unwrap();
    rouille.join().unwrap();
}

fn load_provided_directory(directory: &Path) -> Arc<HashMap<String, Vec<u8>>> {
    Arc::new(file::load_directory(directory))
}