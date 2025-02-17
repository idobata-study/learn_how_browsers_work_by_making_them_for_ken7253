#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::ToString;
use net_wasabi::http::HttpClient;
use noli::prelude::*;

fn main() -> u64 {
    let client = HttpClient::new();
    match client.get("example.com".to_string(), 80, "/".to_string()) {
        Ok(response) => {
            print!("response:\n{:#?}", response);
        }
        Err(e) => {
            print!("error:\n{:#?}", e)
        }
    }

    0
}

entry_point!(main);
