#[macro_use]
extern crate serde_derive;
use linear_map::LinearMap;

/// Warning: Escaped JSON strings cannot be deserialized to &str type fields - values that would need escaping are not supported in this fields.

pub type IndexedHeader = LinearMap<String, Vec<String>>;
pub type RawHeader = Vec<(String, String)>;
pub type LogVarsIndex<'i> = LinearMap<&'i str, String>;

/// This is entry type that JSON varnishslog output format can be deserialized to
#[derive(Deserialize, Debug)]
#[serde(tag = "record_type")]
pub enum HttpAccessRecord<'i> {
    #[serde(rename = "client_request", borrow)]
    ClientRequest(ClientRequest<'i>),
    #[serde(rename = "esi_subrequest", borrow)]
    EsiSubrequest(ClientRequest<'i>),
    #[serde(rename = "pipe_session", borrow)]
    PipeSession(PipeSession<'i>),
}

impl<'i> HttpAccessRecord<'i> {
    pub fn as_client_request(&self) -> Option<&ClientRequest<'i>> {
        if let HttpAccessRecord::ClientRequest(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_esi_subrequest(&self) -> Option<&ClientRequest<'i>> {
        if let HttpAccessRecord::EsiSubrequest(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_pipe_session(&self) -> Option<&PipeSession<'i>> {
        if let HttpAccessRecord::PipeSession(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ClientRequest<'i> {
    pub vxid: u64,
    #[serde(borrow)]
    pub session: Option<SessionInfo<'i>>,
    pub remote_address: Address<'i>,
    pub start_timestamp: f64,
    pub end_timestamp: Option<f64>,
    pub handling: &'i str,
    pub request: Option<HttpRequest>,
    pub response: Option<HttpResponse>,
    #[serde(borrow)]
    pub backend_access: Option<BackendAccess<'i>>,
    pub process_duration: Option<f64>,
    pub fetch_duration: Option<f64>,
    pub ttfb_duration: f64,
    pub serve_duration: f64,
    pub recv_header_bytes: u64,
    pub recv_body_bytes: u64,
    pub recv_total_bytes: u64,
    pub sent_header_bytes: u64,
    pub sent_body_bytes: u64,
    pub sent_total_bytes: u64,
    pub esi_count: u64,
    #[serde(borrow)]
    pub compression: Option<Compression<'i>>,
    pub restart_count: u64,
    pub restart_log: Option<Log<'i>>,
    pub log: Log<'i>,
    pub request_header_index: Option<IndexedHeader>,
    pub response_header_index: Option<IndexedHeader>,
}

#[derive(Debug, Deserialize)]
pub struct Proxy<'i> {
    pub version: &'i str,
    #[serde(borrow)]
    pub client_address: Address<'i>,
    #[serde(borrow)]
    pub server_address: Address<'i>,
}

#[derive(Debug, Deserialize)]
pub struct SessionInfo<'i> {
    pub vxid: u32,
    pub open_timestamp: f64,
    #[serde(borrow)]
    pub local_address: Option<Address<'i>>,
    #[serde(borrow)]
    pub remote_address: Address<'i>,
    #[serde(borrow)]
    pub proxy: Option<Proxy<'i>>,
}

#[derive(Debug, Deserialize)]
pub struct Address<'i> {
    pub ip: &'i str,
    pub port: i64,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Headers {
    Raw(RawHeader),
    Indexed(IndexedHeader),
}

impl Headers {
    pub fn as_raw(&self) -> Option<&RawHeader> {
        if let Headers::Raw(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_indexed(&self) -> Option<&IndexedHeader> {
        if let Headers::Indexed(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct HttpRequest {
    pub protocol: String,
    pub method: String,
    pub url: String,
    pub headers: Headers,
}

#[derive(Debug, Deserialize)]
pub struct HttpResponse {
    pub status: u32,
    pub reason: String,
    pub protocol: String,
    pub headers: Headers,
}

#[derive(Debug, Deserialize)]
pub struct BackendAccess<'i> {
    pub vxid: i64,
    pub start_timestamp: Option<f64>,
    pub end_timestamp: Option<f64>,
    pub handling: &'i str,
    pub request: HttpRequest,
    pub response: Option<HttpResponse>,
    pub send_duration: f64,
    pub wait_duration: Option<f64>,
    pub ttfb_duration: Option<f64>,
    pub fetch_duration: Option<f64>,
    pub sent_header_bytes: Option<u64>,
    pub sent_body_bytes: Option<u64>,
    pub sent_total_bytes: Option<u64>,
    pub recv_header_bytes: Option<u64>,
    pub recv_body_bytes: Option<u64>,
    pub recv_total_bytes: Option<u64>,
    pub retry: u64,
    #[serde(borrow)]
    pub backend_connection: Option<BackendConnection<'i>>,
    #[serde(borrow)]
    pub cache_object: Option<CacheObject<'i>>,
    #[serde(borrow)]
    pub compression: Option<Compression<'i>>,
    pub log: Log<'i>,
    pub request_header_index: Option<IndexedHeader>,
    pub response_header_index: Option<IndexedHeader>,
    pub cache_object_response_header_index: Option<IndexedHeader>,
    pub lru_nuked: u32,
}

#[derive(Debug, Deserialize)]
pub struct PipeSession<'i> {
    pub vxid: u32,
    #[serde(borrow)]
    pub remote_address: Address<'i>,
    pub start_timestamp: f64,
    pub end_timestamp: Option<f64>,
    #[serde(borrow)]
    pub backend_connection: Option<BackendConnection<'i>>,
    pub request: HttpRequest,
    pub backend_request: HttpRequest,
    pub process_duration: Option<f64>,
    pub ttfb_duration: Option<f64>,
    pub recv_total_bytes: u64,
    pub sent_total_bytes: u64,
    pub log: Log<'i>,
    pub request_header_index: Option<IndexedHeader>,
    pub backend_request_header_index: Option<IndexedHeader>,
}

#[derive(Debug, Deserialize)]
pub struct BackendConnection<'i> {
    pub fd: i64,
    pub name: &'i str,
    #[serde(borrow)]
    pub remote_address: Option<Address<'i>>,
    #[serde(borrow)]
    pub local_address: Address<'i>,
}

#[derive(Debug, Deserialize)]
pub struct CacheObject<'i> {
    pub storage_type: &'i str,
    pub storage_name: &'i str,
    pub ttl_duration: Option<f64>,
    pub grace_duration: Option<f64>,
    pub keep_duration: Option<f64>,
    pub since_timestamp: f64,
    pub origin_timestamp: f64,
    pub fetch_mode: Option<&'i str>,
    pub fetch_streamed: Option<bool>,
    pub response: Option<HttpResponse>,
}

#[derive(Debug, Deserialize)]
pub struct Compression<'i> {
    pub operation: &'i str,
    pub bytes_in: u64,
    pub bytes_out: u64,
}

#[derive(Debug, Deserialize)]
pub struct RawLog<'i> {
    pub entry_type: &'i str,
    pub message: String,
    pub detail: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Log<'i> {
    #[serde(borrow)]
    Raw(Vec<RawLog<'i>>),
    #[serde(borrow)]
    Indexed(IndexedLog<'i>),
}

#[derive(Debug, Deserialize)]
pub struct IndexedLog<'i> {
    #[serde(borrow)]
    pub vars: LogVarsIndex<'i>,
    pub messages: Vec<String>,
    pub acl_matched: Vec<&'i str>,
    pub acl_not_matched: Vec<&'i str>,
}

impl<'i> Log<'i> {
    pub fn as_raw(&self) -> Option<&Vec<RawLog<'i>>> {
        if let Log::Raw(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_indexed(&self) -> Option<&IndexedLog<'i>> {
        if let Log::Indexed(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::BufRead;
    use serde_json;

    #[test]
    fn test_parsing() {
        let test_data = File::open("test.json").unwrap();
        for (no, line) in BufReader::new(test_data).lines().enumerate() {
            let line = line.unwrap();
            match serde_json::from_str::<HttpAccessRecord>(&line) {
                Err(err) => panic!("{} [{}]: {}", err, no, line),
                _ => ()
            };
        }
    }
}