use crate::response::Response;
use crate::threaded::server::{ServerStats, RequestResult};
use std::sync::{Arc, MutexGuard};
use std::collections::HashMap;

#[derive(Debug)]
struct ResultView {
    request_successes: Vec<bool>,
    path_counts: HashMap<String, PathCount>
}

impl ResultView {
    fn num_total(&self) -> usize {
        self.request_successes.len()
    }

    fn num_successful(&self) -> usize {
        self.request_successes.iter().filter(|result| **result).collect::<Vec<&bool>>().len()
    }

    fn num_unsuccessful(&self) -> usize {
        self.request_successes.iter().filter(|result| !**result).collect::<Vec<&bool>>().len()
    }
}

#[derive(Debug)]
struct PathCount {
    path: String,
    num_requested: i32
}

impl PathCount {
    fn increase_num(&mut self) {
        self.num_requested += 1
    }
}

pub fn stats_response(stats: Arc<ServerStats>, resources: Arc<HashMap<String, String>>) -> Result<Response, Response> {
    let results = stats.request_results.lock().unwrap();

    let request_successes: Vec<bool> = results.iter().map(|result| result.is_successful()).collect();

    let mut path_counts: HashMap<String, PathCount> = HashMap::new();

    &results.iter().for_each(|result| {
        let resource = (&result.requested_resource).to_string();
        path_counts.entry(resource.to_string()).or_insert(PathCount{path: resource.to_string(), num_requested: 0}).increase_num();
    });

    let result_view = ResultView{request_successes, path_counts};

    let html = build_html(resources, results, result_view);

    let mut response = Response::default_ok();
    &response.add_content_type("_.html".to_string());
    response.body = html.as_bytes().to_vec();

    Ok(response)
}

fn build_html(resources: Arc<HashMap<String, String>>, results: MutexGuard<Vec<RequestResult>>, result_view: ResultView) -> String {
    let mut html = resources.get("/stats.html").unwrap().to_string();

    html = html.replace("{{num_total}}", &result_view.num_total().to_string());
    html = html.replace("{{num_successful}}", &result_view.num_successful().to_string());
    html = html.replace("{{num_unsuccessful}}", &result_view.num_unsuccessful().to_string());

    let table_entry = resources.get("/stats_table_entry.html").unwrap().to_string();
    let mut table_entries = String::new();

    results.iter().for_each(|result| {
        let mut entry = table_entry.to_string();
        entry = entry.replace("{{Time}}", &result.time.to_string());
        entry = entry.replace("{{Path}}", &result.requested_resource);
        entry = entry.replace("{{Code}}", &result.response_code.to_string());
        entry = entry.replace("{{Duration}}", &result.duration.to_string());
        table_entries = format!("{}\n{}", table_entries, entry);
    });

    html = html.replace("{{result_entries}}", &table_entries);

    html
}