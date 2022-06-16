use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RAWLogEntry {
    pub source_ip: String,
    pub request_timestamp: String,
    pub request_http_method: String,
    pub request_endpoint: String,
    pub request_http_version: String,
    pub response_code: i64,
    pub response_bytes_count: i64,
    pub http_client_user_agent: String,
}
