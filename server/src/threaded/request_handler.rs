use crate::request::Request;
use crate::response::Response;
use crate::threaded::controller::stats_controller::stats_response;
use crate::threaded::controller::file_controller::file_response;
use crate::threaded::server::{ServerStats, ServerFiles};
use std::sync::Arc;

/// Mapping endpoints to the corresponding controller actions
pub fn handle_request(request: &Request, server_files: ServerFiles, stats: Arc<ServerStats>) -> Response {
    let path = &request.request_identifiers.path;

    let response = match &path[..] {
        "/stats" => stats_response(stats, server_files.dynamic_files),
        _ => file_response(server_files, path.to_string())
    };

    match response {
        Ok(res) => res,
        Err(res) => res
    }
}