mod roblox_json;
use hyper::body::HttpBody as _;
use hyper::Client;
use hyper_tls::HttpsConnector;
use rand::distributions::Alphanumeric;
use rand::distributions::{Distribution, Uniform};
use rand::{thread_rng, Rng};
use roblox_json::RobloxJSONResponse;
use std::fs;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let mut rng = rand::thread_rng();
    let username_length = Uniform::from(3..20);
    let mut file = fs::File::create("available_usernames.txt")?;

    loop {
        let random_username: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(username_length.sample(&mut rng))
            .map(char::from)
            .collect();

        let url = format!("https://auth.roblox.com/v1/usernames/validate?request.username={}&request.birthday=1337-04-20&request.context=Signup", random_username).parse().unwrap();
        let mut resp = client.get(url).await?;

        let response_bytes = resp.body_mut().data().await.unwrap().unwrap();

        let json: RobloxJSONResponse = RobloxJSONResponse::response_to_json(response_bytes);

        if json.code == 0 {
            println!("Username {} is available for use", random_username);
            file.write_all((random_username + "\n").as_bytes())
                .expect("Unable to write to file");
        } else {
            println!("Username {} is not available for use", random_username);
        }
    }
}
