use std::fs;

use crate::http::{Method, Response, StatusCode};

use super::server::Handler;

pub struct WebsiteHandler {
    pub public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    return fs::read_to_string(path).ok();
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    return None;
                }
            }  
            Err(_) => {return None}
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&self, request: &crate::http::Request) -> super::http::Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                _ => Response::new(StatusCode::NotFound, None),
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
