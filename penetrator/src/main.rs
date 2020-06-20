mod penetrator;

use penetrator::penetrate;

#[tokio::main]
async fn main() {
    penetrate(10, "http://httpbin.org/get".to_string()).await;
}