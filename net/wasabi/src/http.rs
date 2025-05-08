extern crate alloc;
use alloc::format;
use alloc::string::String;
use crate::alloc::string::ToString;
use saba_core::error::Error;
use saba_core::http::HttpResponse;
use noli::net::lookup_host;
use noli::net::SocketAddr;

pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error> {
        let ips = match lookup_host(&host) {
            Ok(ips) => ips,
            Err(e) => {
                return Err(Error::Network(format!("Failed to find IP addresses: {:#?}", e)))
            }
        };

        if ips.len() < 1 {
            return Err(Error::Network("Failed to find IP addresses".to_string()));
        }

        let socket_addr: SocketAddr = (ips[0], port).into();
    }
}

