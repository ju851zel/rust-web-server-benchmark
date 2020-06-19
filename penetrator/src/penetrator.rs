extern crate futures;

use futures::future::join_all;
use std::time::Instant;
use self::futures::{TryFutureExt, FutureExt};

async fn do_request(url: String) -> Result<(u16, u128), reqwest::Error> {
    let start = Instant::now();
    let res = reqwest::get("http://httpbin.org/get").await?;
    //let res = reqwest::get(&url).await?;
    let duration = start.elapsed();

    Ok((res.status().as_u16(), duration.as_millis()))
}

pub async fn penetrate(amount: usize, port: u16) {
    let url = format!("http://127.0.0.1:{}", port);

    let requests = (0..amount)
        .map(|_| do_request(url.clone()));

    let start = Instant::now();
    let request_results = join_all(requests).await;
    let duration = start.elapsed();

    generate_result(request_results, duration.as_millis());
}

fn generate_result(request_results: Vec<Result<(u16, u128), reqwest::Error>>, duration: u128) {
    for request_result in request_results {
        let res = request_result.unwrap();

        println!("Status: {}", res.0);
        println!("Dur: {}", res.1);
    }
}