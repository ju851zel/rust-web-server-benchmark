use crate::request::Request;
use crate::Directory;
use crate::response::Response;
use crate::threaded::controller::stats_controller::stats_response;
use crate::threaded::controller::file_controller::file_response;
use crate::threaded::server::ServerStats;
use std::sync::Arc;
use std::collections::HashMap;

pub fn handle_request(request: &Request, dir: Directory, resources: Arc<HashMap<String, String>>, stats: Arc<ServerStats>) -> Response {

    let path = &request.request_identifiers.path;

    let response = match &path[..] {
        "/stats" => stats_response(stats),
        _ => file_response(dir, path.to_string(), resources)
    };

    match response {
        Ok(res) => res,
        Err(res) => res
    }
}