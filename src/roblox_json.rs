use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RobloxJSONResponse {
    pub code: i32,
    pub message: String,
}

impl RobloxJSONResponse {
    pub fn response_to_json(response: &hyper::body::Bytes) -> Option<Self> {
        match serde_json::from_slice(&response[..]) {
            Ok(json) => Some(json),
            Err(e) => {
                println!("Error: {}", e);
                None
            }
        }
    }
}
