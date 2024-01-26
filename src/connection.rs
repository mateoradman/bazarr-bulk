use std::process::exit;

use reqwest::{Client, Url};

pub async fn check_health(client: &Client, url: &Url) {
    let mut url = url.clone();
    url.path_segments_mut().unwrap().push("system/status");
    let response = client.get(url).send().await;
    if let Ok(res) = response {
        if res.status().is_success() {
            println!("Bazarr API is healthy.");
        } else if res.status() == reqwest::StatusCode::UNAUTHORIZED {
            eprintln!("Unauthorized! Please verify that the correct Bazarr API key has been set in the configuration file.");
            exit(1);
        } else {
            eprintln!(
                "Error while connecting to Bazarr. Response: {}",
                res.text().await.unwrap()
            );
            println!("Attempting to continue anyway...")
        }
    } else {
        println!(
            "Error while connecting to Bazarr. Please verify that the protocol, host, and port provided in the configuration file are correct."
        );
        exit(1);
    }
}
