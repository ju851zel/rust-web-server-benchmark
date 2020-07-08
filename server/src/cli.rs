extern crate clap;

/// Module containing the CLI for the webservers


use clap::{Arg, App, ArgMatches};

/// Starts the CLI and returns:
/// - the address for the server to listen on
/// - the directory the server should serve
/// - the number of threads the thread pool should have
pub fn start_cli() -> (String, i32, String, i32, String) {
    let cli = create_matchers();

    let ip = cli.value_of("ip").unwrap();
    let port = cli.value_of("port").unwrap().parse::<u32>().unwrap();
    let dir = cli.value_of("dir").unwrap();
    let threads = cli.value_of("threads").unwrap().parse::<u32>().unwrap();
    let type_ = cli.value_of("type").unwrap();

    (ip.to_string(),
     port as i32,
     dir.to_string(),
     threads as i32,
     type_.to_string())
}


/// Creates the required CLI parser.
fn create_matchers() -> ArgMatches<'static> {
    return App::new("Webserver")
        .version("0.1.0")
        .author("Jörg S, Julian Z")
        .about("A simple but fast server in rust")
        .arg(Arg::with_name("port")
            .short("p")
            .required(true)
            .long("port")
            .default_value("9000")
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
            .default_value("127.0.0.1")
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
        .arg(Arg::with_name("type")
            .short("y")
            .required(true)
            .long("server_type")
            .default_value("all")
            .validator(|value| valid_type(value))
            .value_name("TYPE")
            .help("The type of the server [threaded|event_loop|rouille|all]. Event loop is only supported on BSD systems, and eventually linux.")
            .takes_value(true))
        .get_matches();
}


/// Validate the correctness of the user provided Port
fn valid_port(string: String) -> Result<(), String> {
    match string.parse::<u32>() {
        Ok(num) if num > 1024 && num < 65536 => { Ok(()) }
        _ => { Err("Please provide a valid port (>1024)".to_string()) }
    }
}

/// Validate the correctness of the user provided Portserver type
fn valid_type(s: String) -> Result<(), String> {
    match &s[..] {
        "threaded" => {}
        "rouille" => {}
        "event_loop" | "all" => {
            match std::env::consts::OS {
                "macos" => {}
                "freebsd" => {}
                "linux" => {}
                _ => return Err("Event loop is only supported on BSD systems, and eventually linux.".to_string())
            }
        }
        _ => return Err("Please select a server type [threaded|event_loop|rouille]".to_string())
    }
    return Ok(());
}

/// Validate the correctness of the user provided amount of threads
fn valid_threads(string: String) -> Result<(), String> {
    match string.parse::<u32>() {
        Ok(num) if num >= 2 => { Ok(()) }
        _ => { Err("Please provide a valid amount of threads (>=2)".to_string()) }
    }
}

/// Validate the correctness of the user provided ip
fn valid_ip(ip: String) -> Result<(), String> {
    let blocks = ip[..].split('.')
        .collect::<Vec<&str>>()
        .iter()
        .filter(|num| num.parse::<u8>().is_ok())
        .count();

    if blocks == 4 {
        Ok(())
    } else {
        Err("Please provide a valid IPv4. e.g. 127.0.0.1".to_string())
    }
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