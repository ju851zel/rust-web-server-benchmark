use crate::response::Response;
use crate::DynamicFiles;

/// Standard dynamic 400 error response
pub fn error_response_400(error_message: String, dynamic_files: DynamicFiles) -> Response {
    let mut response = Response::default_bad_request();
    response.dynamic_error_response(error_message, dynamic_files);
    response
}

/// Standard dynamic 404 error response
pub fn error_response_404(error_message: String, dynamic_files: DynamicFiles) -> Response {
    let mut response = Response::default_not_found();
    response.dynamic_error_response(error_message, dynamic_files);
    response
}

/// Standard dynamic 500 error response
#[allow(dead_code)]
pub fn error_response_500(error_message: String, dynamic_files: DynamicFiles) -> Response {
    let mut response = Response::default_not_found();
    response.dynamic_error_response(error_message, dynamic_files);
    response
}
