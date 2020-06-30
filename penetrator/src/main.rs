mod penetrator;
mod cli;

use penetrator::penetrate;

#[tokio::main]
async fn main() {
    let (url, number_of_requests) = cli::start_cli();

    penetrate(number_of_requests, url).await;

    //not working
    penetrate(1, "http://www.httpvshttps.com/".to_string()).await;
    penetrate(1, "http://www.httpbin.org/get".to_string()).await;
    penetrate(1, "http://123.123.123.23:9000/small.html".to_string()).await;
    //working
    penetrate(10, "https://www.google.de".to_string()).await;
}


// https://cfsamsonbooks.gitbook.io/epoll-kqueue-iocp-explained/the-recipie-for-an-eventqueue/epoll
// https://docs.rs/mio/0.5.1/mio/struct.EventLoop.html
// crossbeam