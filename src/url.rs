use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Url {
    url_hash: String,
    url: String,
}
