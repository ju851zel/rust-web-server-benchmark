# Rust web-servers including benchmarking
The projekt in "Programmieren in Rust". By:
- Jörg Stenger <joergstenger@exozet.com>
- Julian Zellner <ju851zel@htwg-konstanz.de>

The idea is to build two binaries. One including the webservers and one including the "penetrator".
The webserver binary includes the following servers:
- a multi threaded webserver, build arround a threadpool, this is our main server the rest is only for comparison
- a webserver, build with one thread per request
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

## How to build/run the projekt

General information:
- Remember the resources directory must lay in the same directory as the binary
- when running the webserver/penetrator have in mind the maximum of open files of your OS/shell session.


Build the webserver and penetrator binary
```
cargo run build
```
Run the webserver
```
cargo run --release --bin webserver -- --port 9000 --ip_address "127.0.0.1" --directory "path/to/files/to/serve" --server_type "threaded"
```
Run the penetrator
```
cargo run --release --bin penetrator -- --url "http://www,google.de" --number_of_requests 10
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

## Self criticicm
- a stricter outline of our goals and what we wanted to achieve could have helped at the beginning.
- the non blocking server seems not to work to 100%; using unsafe rust makes it more difficult, though was good to try out and learn about nonetheless
- some todos are still in there, e.g. too small buffer for very large html sites
- the multi threaded webserver with the stats endpoint works well and we really like it
- the audio quality of the video does not meet our own requirements
