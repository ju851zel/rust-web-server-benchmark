mod penetrator;

use penetrator::penetrate;

#[tokio::main]
async fn main() {
    penetrate(23, 8008).await;
}