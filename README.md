# rust-web-server-benchmark
The projekt in "Programmieren in Rust".

The idea is to build a webserver in rust. This means we want a binray crate that serves a specified directory as a server. The focus of the server should be performance.

## Specifications
- lightning fast webserver
- serving of static html from a directory
- benchmarking about performance
  - we want to create a second binary that bombs the server with parralel requests
  - we could implement parts of the server with different crates, to see the differences in performance

- when starting the server via the commandline make it possible to specify the following aspects
  - the directory to serve
  - the interface to listen on
  - the port to listen on
  - tbc

## Crates
- we should not use rocket.rs, but there is a library which makes handling tcp connections easier, we could use this one
