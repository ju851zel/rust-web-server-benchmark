use crate::response::Response;
use crate::Directory;
use std::collections::HashMap;
use std::sync::Arc;

pub fn error_response_400(error_message: String, resources: Arc<HashMap<String, String>>) -> Response {
    let mut response = Response::default_bad_request();
    response.dynamic_error_response(error_message, resources);
    response
}

pub fn error_response_404(error_message: String, resources: Arc<HashMap<String, String>>) -> Response {
    let mut response = Response::default_not_found();
    response.dynamic_error_response(error_message, resources);
    response
}

pub fn error_response_500(error_message: String, resources: Arc<HashMap<String, String>>) -> Response {
    let mut response = Response::default_not_found();
    response.dynamic_error_response(error_message, resources);
    response
}
