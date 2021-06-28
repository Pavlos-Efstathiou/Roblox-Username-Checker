use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RobloxJSONResponse {
    pub code: i32,
    pub message: String,
}

impl RobloxJSONResponse {
    pub fn response_to_json(response: hyper::body::Bytes) -> Self {
        serde_json::from_slice(&response[..]).unwrap()
    }
}
