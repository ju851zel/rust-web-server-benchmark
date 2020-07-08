extern crate clap;

use clap::{Arg, App, ArgMatches};
use reqwest::Url;

/// Starts the CLI.
///
/// returns:
/// - the url the penetrator will send requests to
/// - the number of requests to be sent
pub fn start_cli() -> (String, usize) {
    let cli = create_matchers();

    let url = cli.value_of("url").unwrap();
    let number_of_requests = cli.value_of("num").unwrap().parse::<usize>().unwrap();

    (url.to_string(), number_of_requests)
}

/// Creates the required CLI parser.
fn create_matchers() -> ArgMatches<'static> {
    return App::new("Penetrator")
        .version("0.1.0")
        .author("Jörg S, Julian Z")
        .about("A simple website penetrator / performance tester")
        .arg(Arg::with_name("url")
            .required(true)
            .long("url")
            .value_name("URL")
            .validator(|value| valid_url(value))
            .help("The url the penetrator will send requests to")
            .takes_value(true))
        .arg(Arg::with_name("num")
            .required(false)
            .long("number_of_requests")
            .default_value("10")
            .value_name("NUM")
            .validator(|value| valid_num(value))
            .help("The number of requests to be sent")
            .takes_value(true))
        .get_matches();
}


/// Validate the correctness of the user provided URL
fn valid_url(url: String) -> Result<(), String> {
    match Url::parse(&url) {
        Ok(_) => Ok(()),
        Err(_) => Err("Please provide a valid url".to_string())
    }
}

/// Validate the correctness of the user provided number of requests
fn valid_num(num: String) -> Result<(), String> {
    match num.parse::<usize>() {
        Ok(_) => Ok(()),
        Err(_) => Err("Please provide a valid number".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_url_test() {
        assert_eq!(valid_url("https://google.com".to_string()), Ok(()));
        assert_eq!(valid_url("test".to_string()), Err("Please provide a valid url".to_string()));
        assert_eq!(valid_url("http://127.0.0.1:9117/".to_string()), Ok(()));
    }

    #[test]
    fn valid_num_test() {
        assert_eq!(valid_num("".to_string()), Err("Please provide a valid number".to_string()));
        assert_eq!(valid_num("test".to_string()), Err("Please provide a valid number".to_string()));
        assert_eq!(valid_num("0".to_string()), Ok(()));
        assert_eq!(valid_num("1".to_string()), Ok(()));
        assert_eq!(valid_num("2".to_string()), Ok(()));
        assert_eq!(valid_num("3".to_string()), Ok(()));
        assert_eq!(valid_num("8".to_string()), Ok(()));
        assert_eq!(valid_num("1000".to_string()), Ok(()));
    }
}