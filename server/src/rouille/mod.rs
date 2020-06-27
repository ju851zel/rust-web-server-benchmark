use rouille::{Response, ResponseBody};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::ops::Deref;

pub fn start_server(ip: String, port: i32, dir: Arc<HashMap<String, Vec<u8>>>) {
    let address = format!("{}:{}", ip, port);

    println!("Rouille listening for incoming requests on {}", address);

    let requests_counter = Mutex::new(0);

    rouille::start_server(address, move |request| {
        println!("{:#?}", request.url());
        *requests_counter.lock().unwrap() += 1;

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