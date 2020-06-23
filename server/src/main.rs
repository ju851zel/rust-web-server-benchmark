use cli;
use std::sync::Arc;
use std::collections::HashMap;
use std::thread;

mod my_server;
mod rouille;
mod hyper_server;


fn main() {
    let (ip, port, directory, thread_pool_size) = cli::start_cli();

    let files: Arc<HashMap<String, Vec<u8>>> = Arc::new(files::load_directory(directory.as_str()));


    let my_files = files.clone();
    let hyper_files = files.clone();
    let rouille_files = files.clone();
    let my_ip = ip.clone();
    let hyper_ip = ip.clone();
    let rouille_ip = ip.clone();

    let my = thread::spawn( move || {
        my_server::start_server(my_ip, port, thread_pool_size, my_files);
    });

    let hyper = thread::spawn( move || {
        hyper_server::start_server(hyper_ip, port + 1, hyper_files);
    });

    let rouille = thread::spawn( move || {
        rouille::start_server(rouille_ip, port + 2, rouille_files);
    });

    my.join().unwrap();
    hyper.join().unwrap();
    rouille.join().unwrap();
}
