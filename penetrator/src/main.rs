mod penetrator;
mod cli;

use penetrator::penetrate;

#[tokio::main]
async fn main() {
    let (url, number_of_requests) = cli::start_cli();

    penetrate(number_of_requests, url).await;
}