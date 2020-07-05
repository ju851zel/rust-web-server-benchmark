use crate::response::Response;
use crate::threaded::server::{ServerStats, RequestResult};
use std::sync::Arc;

pub fn stats_response(stats: Arc<ServerStats>) -> Result<Response, Response> {
    let mut response = Response::default_ok();

    let results = stats.request_results.lock().unwrap();

    &response.add_content_type("_.html".to_string());
    let mut list = "<ul>".to_string();

    for result in results.iter() {
        list = format!("{}{}", list, &stats_html(result));
    }
    list = format!("{}{}", list, "</ul>");

    response.body = list.as_bytes().to_vec();

    Ok(response)
}

fn stats_html(request_result: &RequestResult) -> String {
    format!("\
        <li>
            <p>Requested Path: {}<p>
            <p>Response Code: {}<p>
            <p>Handling Time: {}<p>
            <p>Request Date: {}<p>
        \
        </li>\
    ", request_result.requested_resource, request_result.response_code, request_result.response_time, request_result.time)
}