use std::io::Write;
use std::{error::Error, io::Read, net::TcpStream};
use std::str;
use crate::lispeum;
use crate::lispeum::environment::Environment;
use crate::lispeum::evaluator::evaluate;
use crate::lispeum::special::SpecialFunctions;

use super::parser::{parse_query_string, parse_request_line};

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = str::from_utf8(&buffer).unwrap();
            let (method, path) = parse_request_line(request);

            if method == "GET" {
                let params = parse_query_string(&path);
                // Example usage of params
                if let Some(topic) = params.get("topic") {
                    let response = match topic.as_str() {
                        "code" => {
                            if let Some(script) = params.get("script") {
                                let mut env = Environment::new(); // Assume Environment is ready to use
                                let special_funcs = SpecialFunctions::new(); // Assume SpecialFunctions is ready to use
                                
                                // Assume script is a String that needs to be parsed into Expr
                                // and parse_lispeum_string is a function that does this.
                                let decoded_script = script.replace("%20", " ");
                                match lispeum::parse_lispeum_string(&decoded_script) {
                                    Ok(expr) => {
                                        match evaluate(expr, &mut env, &special_funcs) {
                                            Ok(result) => {
                                                // Assuming Expr implements Display
                                                format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", result.to_string().len(), result)
                                                
                                            },
                                            Err(e) => {
                                                let error_message = format!("Error evaluating script: {}", e);
                                                format!("HTTP/1.1 400 Bad Request\r\nContent-Length: {}\r\n\r\n{}", error_message.len(), error_message)
                                                
                                            },
                                        }
                                    },
                                    Err(_) => {
                                        "HTTP/1.1 400 Bad Request\r\n\r\nFailed to parse script".to_string()
                                        
                                    },
                                }
                            } else {
                                "HTTP/1.1 400 Bad Request\r\n\r\nMissing 'script' parameter".to_string()
                            }
                        },
                        _ => "HTTP/1.1 404 Not Found\r\n\r\nUnsupported topic".to_string()
                    };
                    stream.write_all(response.as_bytes()).expect("Failed to write response");
                } else {
                    let response = "HTTP/1.1 400 Bad Request\r\n\r\nMissing 'topic' parameter";
                    stream.write_all(response.as_bytes()).expect("Failed to write response");
                }
            }
        },
        Err(e) => println!("Failed to read from connection: {}", e),
    }
}