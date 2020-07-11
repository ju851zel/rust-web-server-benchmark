use crate::response::Response;
use crate::threaded::controller::error_controller::error_response_404;
use crate::threaded::server::ServerFiles;

/// Endpoint that serves a static file
pub fn file_response(server_files: ServerFiles, path: String) -> Result<Response, Response> {
    match server_files.static_files.get(&path) {
        Some(resource) => {
            let mut response = Response::default_ok();
            &response.add_content_type(path);
            response.body = resource.clone();
            Ok(response)
        }
        None => Err(error_response_404(format!("Requested resource {} could not be found.", path), server_files.dynamic_files))
    }
}