#![feature(test)]

extern crate test;
extern crate futures;

use futures::stream;
use futures::future::join_all;

async fn do_request(url: String) -> Result<u16, reqwest::Error> {
    let res = reqwest::get("http://httpbin.org/get").await?;
    //let res = reqwest::get(&url).await?;
    println!("Status: {}", res.status());
    // println!("Headers:\n{:#?}", res.headers());

    //let body = res.text().await?;
    //println!("Body:\n{}", body);

    Ok(res.status().as_u16())
}

async fn bench_requests(amount: usize, port: u16) {
    let url = format!("http://127.0.0.1:{}", port);

    let requests = (0..amount)
        .map(|_| do_request(url.clone()));

    let request_results = join_all(requests);
    request_results.await;
}

#[cfg(test)]
mod tests {
    use crate::bench_requests;

    #[bench]
    fn c_100_requests(b: &mut test::Bencher) {
        bench_requests(1, 8080);
    }

    #[tokio::test]
    async fn grejgre() {
        bench_requests(100, 8080).await;
    }
}