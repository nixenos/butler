use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct RAWLogEntry {
    remote_address: String,
     
}
