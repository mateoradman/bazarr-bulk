use std::process::exit;

use reqwest::Url;
use reqwest_middleware::ClientWithMiddleware;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct BazarrStatusData {
    pub bazarr_version: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BazarrStatus {
    pub data: BazarrStatusData,
}

pub async fn check_health(client: &ClientWithMiddleware, url: &Url) {
    let mut url = url.clone();
    url.path_segments_mut().unwrap().push("system/status");
    let response = client.get(url).send().await;
    match response {
        Ok(res) => {
            if res.status().is_success() {
                let json: Result<BazarrStatus, reqwest::Error> = res.json().await;
                match json {
                    Ok(json) => {
                        if json.data.bazarr_version.is_some() {
                            println!("Bazarr API is healthy.");
                        }
                    }
                    Err(_) => {
                        eprintln!("Error while connecting to Bazarr");
                        exit(1);
                    }
                }
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
