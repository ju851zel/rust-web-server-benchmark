use rouille::{Response, ResponseBody};
use std::collections::HashMap;
use std::sync::Arc;
use crate::Directory;

pub fn start_server(ip: String, port: i32, dir: Directory) {
    let address = format!("{}:{}", ip, port);

    rouille::start_server(address, move |request| {
        let result = match dir.get(request.url().as_str()) {
            Some(value) => (200, value.to_vec()),
            None => (404, vec![]),
        };

        Response {
            status_code: result.0 as u16,
            headers: vec![],
            data: ResponseBody::from_data(result.1)
        }
    });
}