use std::process::exit;

use reqwest::Url;
use reqwest_middleware::ClientWithMiddleware;

pub async fn check_health(client: &ClientWithMiddleware, url: &Url) {
    let mut url = url.clone();
    url.path_segments_mut().unwrap().push("system/status");
    let response = client.get(url).send().await;
    match response {
        Ok(res) => {
            if res.status().is_success() {
                println!("Bazarr API is healthy.");
            } else if res.status() == reqwest::StatusCode::UNAUTHORIZED {
                eprintln!(
                    "Unauthorized request! 
                    Please verify that the correct Bazarr API key has been set in the configuration file."
                );
                exit(1);
            } else {
                eprintln!(
                    "Error while connecting to Bazarr. Response: {}",
                    res.text().await.unwrap()
                );
                println!("Attempting to continue anyway...")
            }
        }
        Err(_) => {
            println!(
                "Unable to establish connection to Bazarr. 
                Please verify that the protocol, host, and port provided in the configuration file are correct."
            );
            exit(1);
        }
    }
}
