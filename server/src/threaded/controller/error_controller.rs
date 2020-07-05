use crate::response::Response;

pub fn error_response_400(error_message: String) -> Response {
    let mut response = Response::default_bad_request();
    response.dynamic_error_response(error_message);
    response
}

pub fn error_response_404(error_message: String) -> Response {
    let mut response = Response::default_not_found();
    response.dynamic_error_response(error_message);
    response
}

pub fn error_response_500(error_message: String) -> Response {
    let mut response = Response::default_not_found();
    response.dynamic_error_response(error_message);
    response
}
