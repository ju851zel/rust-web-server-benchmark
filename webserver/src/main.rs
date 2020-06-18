use std::path::Path;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::sync::Arc;

mod cli;
mod file_util;
mod server;
mod requests;

fn main() {
    let (address, directory, thread_pool_size) = cli::start_cli();

    let files: Arc<HashMap<String, String>> = Arc::new(load_directory(directory));

    server::start_server(address.as_str(), thread_pool_size as usize, files);
}


// todo finish loading of directory in memory and returning it
fn load_directory(path: String) -> HashMap<String, String> {
    match file_util::get_all_files_in_dir(Path::new(path.as_str())) {
        Err(err) => {
            println!("Error getting files in dir: {:?}", err);
            panic!(); //todo
        }
        Ok(list) => {
            println!("Successfully read dir in memory: {:#?}", list.keys());
            return list;
        }
    }
}