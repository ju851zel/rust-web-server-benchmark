# Rust web-servers including benchmarking
The projekt in "Programmieren in Rust". By:
- Jörg Stenger <joergstenger@exozet.com>
- Julian Zellner <ju851zel@htwg-konstanz.de>

The idea is to build two binaries. One including the webservers and one including the "penetrator".
The webserver binary includes the following servers:
- a multi threaded webserver, build arround a threadpool
- a multi threaded webserver, build with one thread per request
- a single threded non blocking webserver, (kind of an event queue), only working on BSD systems!!!

The webserver binary accepts different command line arguments. Run the server with -h to see all options. e.g:
- port
- interface to run on
- how many threads to run in the thread pool
- the path to the directory to serve
- what server to run (threadpool, single thread per request, event loop)
This means we want a binray crate that serves a specified directory as a server. The server handles request either with a threadpool, a eventloop or with a crate called rouille(which creates a thread per request).

The penetrator binary can be used to send multiple requests to a URL. It was used to test our servers. It is also a small benchmarking tool.

The penetrator accepts the following command line arguments. Again use -h to see all options:
- the URL to send the requests to
- the number of requests that should be sent


How to build/run the projekt

Build the webserver and penetrator binary
```
cargo run build
```
Run the webserver
```
cargo run --release --bin webserver -- --port 9000 --ip_address "127.0.0.1" --directory "path/to/fir/to/serve" --server_type "threaded"
```
Run the penetrator
```
cargo run --release --bin penetrator -- --url "http://www,google.de" --number_of_requests 10```
```
Create the docs
```
cargo doc --open --no-deps
```
Build the webserver
```
cargo build --release --bin webserver
```
Build the penetrator
```
cargo build --release --bin penetrator
```
