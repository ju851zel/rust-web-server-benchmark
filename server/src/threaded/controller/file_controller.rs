use crate::response::Response;
use crate::Directory;
use crate::threaded::controller::error_controller::error_response_404;
use std::collections::HashMap;
use std::sync::Arc;

pub fn file_response(dir: Directory, path: String, resources: Arc<HashMap<String, String>>) -> Result<Response, Response> {
    match dir.get(&path) {
        Some(resource) => {
            let mut response = Response::default_ok();
            &response.add_content_type(path);
            response.body = resource.clone();
            Ok(response)
        }
        None => {
            match resources.get(&path) {
                Some(resource) => {
                    let mut response = Response::default_ok();
                    &response.add_content_type(path);
                    response.body = resource.to_string().as_bytes().to_vec();
                    Ok(response)
                }
                None => Err(error_response_404(format!("Requested resource {} could not be found.", path), resources))
            }
        }
    }
}