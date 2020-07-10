use crate::request::Request;
use crate::StaticFiles;
use crate::response::Response;
use crate::threaded::controller::stats_controller::stats_response;
use crate::threaded::controller::file_controller::file_response;
use crate::threaded::server::ServerStats;
use std::sync::Arc;
use std::collections::HashMap;

/// Mapping endpoints to the corresponding controller actions
pub fn handle_request(request: &Request, dir: StaticFiles, resources: Arc<HashMap<String, String>>, stats: Arc<ServerStats>) -> Response {
    let path = &request.request_identifiers.path;

    let response = match &path[..] {
        "/stats" => stats_response(stats, resources),
        _ => file_response(dir, path.to_string(), resources)
    };

    match response {
        Ok(res) => res,
        Err(res) => res
    }
}