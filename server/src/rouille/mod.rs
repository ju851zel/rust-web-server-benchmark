use rouille::{Response, ResponseBody};
use std::collections::HashMap;
use std::sync::Arc;
use files;

pub fn start_server(ip: String, port: i32, dir: Arc<HashMap<String, String>>) {
    let address = format!("{}:{}", ip, port);

    println!("Rouille listening for incoming requests on {}", address);

    rouille::start_server(address, move |request| {
        println!("{:#?}", request.url());

        let result = match dir.get(request.url().as_str()) {
            Some(value) => (200, value.to_string()),
            None => (404, "The requested ressource was not found".to_string()),
        };


        Response {
            status_code: result.0 as u16,
            headers: vec![],
            data: ResponseBody::from_string(result.1),
        }
    });
}