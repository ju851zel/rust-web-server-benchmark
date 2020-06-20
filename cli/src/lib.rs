extern crate clap;

use clap::{Arg, App, ArgMatches};

/// Starts the CLI and returns:
/// - the address for the server to listen on
/// - the directory the server should serve
/// - the number of threads the thread pool should have
pub fn start_cli() -> (String, String, u32) {
    let cli = create_matchers();

    let ip = cli.value_of("ip").unwrap();
    let port = cli.value_of("port").unwrap().parse::<u32>().unwrap();
    let dir = cli.value_of("dir").unwrap();
    let threads = cli.value_of("threads").unwrap().parse::<u32>().unwrap();
    println!("Serving directory: {}", dir);
    println!("Listening on {}:{} with {} threads", ip, port, threads);

    (format!("{}:{}", ip, port),
     format!("{}", dir),
     threads)
}


fn create_matchers() -> ArgMatches<'static> {
    return App::new("Webserver")
        .version("0.1.0")
        .author("Jörg S, Julian Z")
        .about("A simple but fast webserver in rust")
        .arg(Arg::with_name("port")
            .short("p")
            .required(true)
            .long("port")
            .default_value("8080")
            .value_name("PORT")
            .validator(|value| valid_port(value))
            .help("The port the server will listen on")
            .takes_value(true))
        .arg(Arg::with_name("threads")
            .short("t")
            .required(true)
            .long("threads")
            .default_value("8")
            .value_name("THREADS")
            .validator(|value| valid_threads(value))
            .help("The amount of threads to handle the requests")
            .takes_value(true))
        .arg(Arg::with_name("ip")
            .short("ip")
            .required(true)
            .long("ip_address")
            .default_value("localhost")
            .validator(|value| valid_ip(value))
            .value_name("IP")
            .help("The IPv4 the server will listen on")
            .takes_value(true))
        .arg(Arg::with_name("dir")
            .short("d")
            .required(true)
            .long("directory")
            .default_value("_dist")
            .value_name("DIR")
            .help("The directory the server should serve")
            .takes_value(true))
        .get_matches();
}


fn valid_port(string: String) -> Result<(), String> {
    return match string.parse::<u32>() {
        Ok(num) if num > 1024 && num < 65536 => { Ok(()) }
        _ => { Err("Please provide a valid port".to_string()) }
    };
}

fn valid_threads(string: String) -> Result<(), String> {
    return match string.parse::<u32>() {
        Ok(num) if num > 2 => { Ok(()) }
        _ => { Err("Please provide a valid amount of threads".to_string()) }
    };
}


fn valid_ip(ip: String) -> Result<(), String> {
    let blocks = ip[..].split('.')
        .collect::<Vec<&str>>()
        .iter()
        .filter(|num| num.parse::<u8>().is_ok())
        .count();

    if blocks == 4 || ip == "localhost".to_string() {
        Ok(())
    } else { Err("Please provide a valid IPv4. e.g. 127.0.0.1".to_string()) }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_port_test() {
        assert_ne!(valid_port("".to_string()), Ok(()));
        assert_ne!(valid_port("test".to_string()), Ok(()));
        assert_ne!(valid_port("-1".to_string()), Ok(()));
        assert_ne!(valid_port("80".to_string()), Ok(()));
        assert_ne!(valid_port("1024".to_string()), Ok(()));
        assert_eq!(valid_port("1025".to_string()), Ok(()));
        assert_eq!(valid_port("9000".to_string()), Ok(()));
        assert_eq!(valid_port("65535".to_string()), Ok(()));
        assert_ne!(valid_port("65536".to_string()), Ok(()));
        assert_ne!(valid_port("1000000".to_string()), Ok(()));
    }

    #[test]
    fn valid_thread_test() {
        assert_ne!(valid_threads("".to_string()), Ok(()));
        assert_ne!(valid_threads("test".to_string()), Ok(()));
        assert_ne!(valid_threads("0".to_string()), Ok(()));
        assert_ne!(valid_threads("1".to_string()), Ok(()));
        assert_ne!(valid_threads("2".to_string()), Ok(()));
        assert_eq!(valid_threads("3".to_string()), Ok(()));
        assert_eq!(valid_threads("8".to_string()), Ok(()));
        assert_eq!(valid_threads("1000".to_string()), Ok(()));
    }

    #[test]
    fn valid_ip_test() {
        assert_ne!(valid_ip("".to_string()), Ok(()));
        assert_ne!(valid_ip("test".to_string()), Ok(()));
        assert_ne!(valid_ip("-10".to_string()), Ok(()));
        assert_ne!(valid_ip("0.2.3.4.4".to_string()), Ok(()));
        assert_ne!(valid_ip("256.4.4.3".to_string()), Ok(()));
        assert_eq!(valid_ip("127.0.0.1".to_string()), Ok(()));
        assert_eq!(valid_ip("192.178.168.17".to_string()), Ok(()));
        assert_ne!(valid_ip("1000000000".to_string()), Ok(()));
    }
}