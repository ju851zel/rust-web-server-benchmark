use crate::request::Request;
use crate::Directory;
use crate::response::Response;
use crate::threaded::controller::stats_controller::stats_response;
use crate::threaded::controller::file_controller::file_response;
use crate::threaded::server::ServerStats;
use std::sync::Arc;

/// Determine if the requested path equals a file in the provided directory or the stats endpoint
pub fn handle_request(request: &Request, dir: Directory, stats: Arc<ServerStats>) -> Response {

    let path = &request.request_identifiers.path;

    let response = match &path[..] {
        "/stats" => stats_response(stats),
        _ => file_response(dir, path.to_string())
    };

    match response {
        Ok(res) => res,
        Err(res) => res
    }
}