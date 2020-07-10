extern crate futures;

use futures::future::join_all;
use std::time::Instant;
use colored::Colorize;

mod cli;

#[tokio::main]
async fn main() {
    let (url, number_of_requests) = cli::start_cli();

    penetrate(number_of_requests, url).await;
}


/// Penetrates the url with the amount of requests provided by the user.
pub async fn penetrate(num_of_requests: usize, url: String) {
    let requests = (0..num_of_requests)
        .map(|_| do_request(&url[..]));

    println!("Sending requests...");
    let start = Instant::now();
    let request_results = join_all(requests).await;
    let duration = start.elapsed();

    generate_result(request_results, duration.as_millis(), url);
}


/// Send a specific request to the url, measuring the time it took.
async fn do_request(url: &str) -> Result<(u16, u128), reqwest::Error> {
    let start = Instant::now();
    let res = reqwest::get(url).await?;
    let duration = start.elapsed();

    Ok((res.status().as_u16(), duration.as_millis()))
}

/// Generates the result over all sent requests.
fn generate_result(request_results: Vec<Result<(u16, u128), reqwest::Error>>, duration: u128, url: String) {
    let num_of_requests = &request_results.len();

    let success: Vec<(u16, u128)> = request_results
        .into_iter()
        .filter(|r| {
            let res = r.as_ref().unwrap_or(&(0, 0)).0;
            r.is_ok() && 200 <= res && 300 > res
        })
        .map(|r| r.unwrap_or((0, 0)))
        .collect();

    let num_of_success = success.len();
    let num_of_errors = num_of_requests - num_of_success;

    let response_times: Vec<u128> = success.into_iter().map(|res| res.1).collect();


    let max_response_time = match response_times.iter().max() {
        Some(max) => max.to_string(),
        _ => "------".to_string()
    };
    let min_response_time = match response_times.iter().min() {
        Some(min) => min.to_string(),
        _ => "------".to_string()
    };
    let sum_response_time: u128 = response_times.iter().sum();

    println!("Fired {} simultaneous requests against {}.", num_of_requests.to_string().cyan(), url);
    println!("It took {}ms for all responses to come in.", duration.to_string().cyan());
    println!("Out of the {} requests, {} of them were successful, {} were unsuccessful.",
             num_of_requests.to_string().cyan(),
             num_of_success.to_string().green(),
             num_of_errors.to_string().red());
    println!("Thus {}% of requests were successful.", (num_of_success / *num_of_requests * 100).to_string().yellow());
    println!("The slowest response time was {}ms.", max_response_time.red());
    println!("The fastest response time was {}ms.", min_response_time.green());
    if num_of_success != 0 {
        println!("The average response time was {}ms.", (sum_response_time / num_of_success as u128).to_string().yellow());
    }
}