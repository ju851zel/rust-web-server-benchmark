extern crate futures;

use reqwest::header;
use reqwest::Client;
use futures::future::join_all;
use std::time::Instant;
use colored::Colorize;

async fn do_request(url: String) -> Result<(u16, u128), reqwest::Error> {
    let start = Instant::now();
    let res = reqwest::get(&url).await?;
    let duration = start.elapsed();

    Ok((res.status().as_u16(), duration.as_millis()))
}

pub async fn penetrate(num_of_requests: usize, url: String) {
    let requests = (0..num_of_requests)
        .map(|_| do_request(url.clone()));

    println!("Sending requests...");
    let start = Instant::now();
    let request_results = join_all(requests).await;
    let duration = start.elapsed();

    generate_result(request_results, duration.as_millis(), url);
}

fn generate_result(request_results: Vec<Result<(u16, u128), reqwest::Error>>, duration: u128, url: String) {
    let num_of_requests = &request_results.len();
    println!("{:#?}", request_results);

    let success: Vec<&(u16, u128)> = request_results.iter()
        .filter(|&r| r.is_ok() && (200 <= r.as_ref().unwrap_or(&(0, 0)).0) && 300 > r.as_ref().unwrap_or(&(0, 0)).0)
        .map(|r| r.as_ref().unwrap_or(&(0, 0)))
        .collect();

    let num_of_success = success.len();
    let num_of_errors = num_of_requests - num_of_success;

    let response_times: Vec<u128> = success.iter().map(|&&r| r.1).collect();

    let max_response_time = response_times.iter().max().unwrap_or(&500000);
    let min_response_time = response_times.iter().min().unwrap_or(&500000);
    let sum_response_time: u128 = response_times.iter().sum();

    println!("Fired {} simultaneous requests against {}.", num_of_requests.to_string().cyan(), url);
    println!("It took {}ms for all responses to come in.", duration.to_string().cyan());
    println!("Out of the {} requests, {} of them were successful, {} were unsuccessful.", num_of_requests.to_string().cyan(), num_of_success.to_string().green(), num_of_errors.to_string().red());
    println!("Thus {}% of requests were successful.", (num_of_success / *num_of_requests * 100).to_string().yellow());
    println!("The slowest response time was {}ms.", max_response_time.to_string().red());
    println!("The fastest response time was {}ms.", min_response_time.to_string().green());
    if num_of_success != 0 {
        println!("The average response time was {}ms.", (sum_response_time / num_of_success as u128).to_string().yellow());
    }
}