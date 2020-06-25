use cli;
use std::sync::Arc;
use std::collections::HashMap;
use std::thread;

mod threaded;
mod rouille;
mod nonblocking;
mod request;
mod response;


fn main() {
    let (ip, port, directory, thread_pool_size) = cli::start_cli();

    let files: Arc<HashMap<String, Vec<u8>>> = Arc::new(files::load_directory(directory.as_str()));


    let my_files = files.clone();
    let rouille_files = files.clone();
    let non_blocking_files = files.clone();
    let my_ip = ip.clone();
    let rouille_ip = ip.clone();
    let non_blocking_ip = ip.clone();

    let my = thread::spawn( move || {
        threaded::start_server(my_ip, port, thread_pool_size, my_files);
    });

    let rouille = thread::spawn( move || {
        rouille::start_server(rouille_ip, port + 2, rouille_files);
    });

    let non_blocking = thread::spawn( move || {
        nonblocking::start_server(non_blocking_ip, port + 1, non_blocking_files);
    });

    my.join().unwrap();
    non_blocking.join().unwrap();
    rouille.join().unwrap();
}
