mod penetrator;

use penetrator::penetrate;

#[tokio::main]
async fn main() {
    //todo not working
    penetrate(10, "http://127.0.0.1:8080/small.html".to_string()).await;

}