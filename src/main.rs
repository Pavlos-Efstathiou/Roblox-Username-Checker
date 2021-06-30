mod roblox_json;
use hyper::body::Bytes;
use hyper::body::HttpBody as _;
use hyper::Client;
use hyper::Uri;
use hyper_tls::HttpsConnector;
use rand::distributions::Alphanumeric;
use rand::distributions::{Distribution, Uniform};
use rand::{thread_rng, Rng};
use roblox_json::RobloxJSONResponse;
use std::fs;
use std::io::prelude::*;
use std::process::exit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let mut rng = rand::thread_rng();
    let username_length = Uniform::from(3..20);
    let mut file = fs::File::create("available_usernames.txt")?;
    let mut err_count: u32 = 0;

    loop {
        let random_username: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(username_length.sample(&mut rng))
            .map(char::from)
            .collect();

        let url: Uri = format!("https://auth.roblox.com/v1/usernames/validate?request.username={}&request.birthday=1337-04-20&request.context=Signup", random_username).parse().unwrap();
        let mut resp = client.get(url).await?;
        let response_bytes = if let Some(x) = resp.body_mut().data().await {
            match x {
                Ok(res) => res,
                Err(e) => {
                    println!("Error: {}", e);
                    Bytes::from(String::from(""))
                }
            }
        } else {
            Bytes::from(String::from(""))
        };

        let json_option = RobloxJSONResponse::response_to_json(&response_bytes);

        if let Some(json) = json_option {
            if json.code == 0 {
                println!(
                    "Username \x1b[1m{}\x1b[0m is available for use",
                    random_username
                );
                file.write_all((random_username + "\n").as_bytes())?;
            } else {
                println!(
                    "\x1b[1m\x1b[31mUsername {} is not available for use\x1b[0m",
                    random_username
                );
            }
        } else {
            err_count += 1;
        };

        if err_count >= 5 {
            exit(1);
        }
    }
}
