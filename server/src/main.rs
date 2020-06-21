use cli;
use std::sync::Arc;
use std::collections::HashMap;
use std::thread;

mod my_server;
mod hyper_server;


fn main() {
    let (ip, port, directory, thread_pool_size) = cli::start_cli();

    let files: Arc<HashMap<String, String>> = Arc::new(files::load_directory(directory.as_str()));


    let my_files = files.clone();
    let hyper_files = files.clone();
    let my_ip = ip.clone();
    let my = thread::spawn( move || {
        my_server::start_server(my_ip, port, thread_pool_size, my_files);
    });

    hyper_server::start_server(ip, port + 1, hyper_files);

    my.join().unwrap();
}
