use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::Path;
use crate::threadpool::ThreadPool;


const IP: &str = "localhost";
const PORT: u32 = 8080;


mod file_util;
mod threadpool;

fn main() {
    let address = format!("{}:{}", IP, PORT);
    let listener = TcpListener::bind(address).unwrap();

    let pool = match ThreadPool::new(8) {
        Err(err) => { panic!(err) }
        Ok(pool) => pool
    };

    // match file_util::get_all_files_in_dir(Path::new("dist")) {
    //     Err(err) => { println!("{:?}", err) }
    //     Ok(list) => println!("{:#?}", list)
    // };


    println!("Listening for incoming requests on {}:{}", IP, PORT);
    for stream in listener.incoming() {
        let connection = stream.unwrap();
        pool.execute(|| {
            handle_connection(connection);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    let response = "HTTP/1.1 200 OK\r\n\r\nhello";

    stream.read(&mut buffer).unwrap();
    println!("Request came in,sending response {}", response);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn get_all_files_in_dir() {
        assert_eq!(bad_add(1, 2), 3);
    }
}