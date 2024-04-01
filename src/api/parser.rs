use std::collections::HashMap;

pub fn parse_request_line(request: &str) -> (String, String) {
    let lines: Vec<&str> = request.lines().collect();
    if let Some(first_line) = lines.get(0) {
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() > 2 {
            return (parts[0].to_string(), parts[1].to_string());
        }
    }
    ("".to_string(), "".to_string())
}

pub fn parse_query_string(query: &str) -> HashMap<String, String> {
    let mut params: HashMap<String, String> = HashMap::new();
    if let Some(query_start) = query.find('?') {
        let queries = &query[(query_start + 1)..];
        for param in queries.split('&') {
            let pair: Vec<&str> = param.split('=').collect();
            if pair.len() == 2 {
                params.insert(pair[0].to_string(), pair[1].to_string());
            }
        }
    }
    params
}