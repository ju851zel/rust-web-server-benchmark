use std::path::Path;

mod cli;
mod file_util;
mod server;
mod requests;

fn main() {
    let (address, directory, thread_pool_size) = cli::start_cli();

    let _directory = load_directory(directory);

    server::start_server(address.as_str(), thread_pool_size as usize);
}


// todo finish loading of directory in memory and returning it
fn load_directory(path: String) {
    match file_util::get_all_files_in_dir(Path::new(path.as_str())) {
        Err(err) => { println!("Error getting files in dir: {:?}", err) }
        Ok(list) => println!("Successfully read dir in memory: {:#?}", list.keys())
    };
}