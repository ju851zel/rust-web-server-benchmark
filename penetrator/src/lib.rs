#![feature(test)]

extern crate test;
extern crate hyper;
extern crate futures;

use futures::{StreamExt, TryFutureExt, Future};
use futures::stream;
use hyper::client::ResponseFuture;
use hyper::{Error, Response};


async fn bench_requests(amount: usize, concurrency: usize, port: u16) {
    let client = hyper::Client::new();

    println!("111111111111111111111111111111111");

    let responses = stream::repeat(()).take(amount)
        .map(|_| {
            let client = &client;
            let url: hyper::Uri = format!("http://127.0.0.1:{}/get", port).parse().unwrap();
            async move {
                client.get(url.clone()).await
            }
        })
        .buffer_unordered(concurrency);

    println!("---------------------------------------------");

    responses.for_each(|b| {
        async {
            match b {
                Response => println!("Got bytes"),
                Error => eprintln!("Got an error: "),
            }
        }
    });

    println!("+++++++++++++++++++++++++++++++++++++++");

//    // Target is localhost with the port of the proxy under test.
//    let url: hyper::Uri = format!("http://127.0.0.1:{}/get", proxy_port)
//        .parse()
//        .unwrap();

//    // This is the benchmark loop that will be executed multiple times and
//    // measured.
//    b.iter(move || {
//        // Build a list of futures that we will execute all at once in parallel
//        // in the end.
//        let mut parallel = Vec::new();
//        for _i in 0..concurrency {
//            // A future that sends requests sequentially by scheduling itself in
//            // a loop-like way.
//            let requests_til_done = loop_fn(0, |counter| {
//                client
//                    .get(url.clone())
//                    .and_then(|res| {
//                        assert_eq!(
//                            res.status(),
//                            hyper::StatusCode::Ok,
//                            "Did not receive a 200 HTTP status code. Make sure Varnish is configured on port 6081 and the backend port is set to 9091 in /etc/varnish/default.vcl. Make sure the backend server is running with `cargo run --example hello_9091` and Rustnish with `cargo run --release --example rustnish_9090`.");
//                        // Read response body until the end.
//                        res.body().for_each(|_chunk| Ok(()))
//                    })
//                    // Break condition of the future "loop". The return values
//                    // signal the loop future if it should run another iteration
//                    // or not.
//                    .and_then(move |_| {
//                        if counter < (amount / concurrency) {
//                            Ok(Loop::Continue(counter + 1))
//                        } else {
//                            Ok(Loop::Break(counter))
//                        }
//                    })
//            });
//            parallel.push(requests_til_done);
//        }
//
//        // The execution should finish when all futures are done.
//        let work = join_all(parallel);
//        // Now run it! Up to this point no request has been sent, we just
//        // assembled heavily nested futures so far.
//        core.run(work).unwrap();
//    });
}

#[cfg(test)]
mod tests {
    use crate::bench_requests;

//    #[bench]
//    fn c_100_requests(b: &mut test::Bencher) {
//        println!("----------------");
//        b.iter(||{
//            println!("--------------");
//            bench_requests(1, 1, 8080);
//        });
//    }

    #[test]
    fn grejgre() {
        println!("--------------");
        bench_requests(1, 1, 8080);
    }

//    #[bench]
//    fn c_100_requests(b: &mut test::Bencher) {
//        bench_requests(b, 100, 1, 9090);
//    }
//
//    #[bench]
//    fn c_100_requests_varnish(b: &mut test::Bencher) {
//        bench_requests(b, 100, 1, 6081);
//    }
}