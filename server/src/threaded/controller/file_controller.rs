use crate::response::Response;
use crate::Directory;
use crate::threaded::controller::error_controller::error_response_404;

pub fn file_response(dir: Directory, path: String) -> Result<Response, Response> {
    match dir.get(&path) {
        Some(resource) => {
            let mut response = Response::default_ok();
            &response.add_content_type(path);
            response.body = resource.clone();
            Ok(response)
        }
        None => Err(error_response_404(format!("Requested resource {} could not be found.", path)))
    }
}