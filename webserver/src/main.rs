use std::path::Path;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::sync::Arc;
use cli;
use files;

mod server;
mod requests;

fn main() {
    let (address, directory, thread_pool_size) = cli::start_cli();

    let files: Arc<HashMap<String, String>> = Arc::new(files::load_directory(directory));

    server::start_server(address.as_str(), thread_pool_size as usize, files);
}
