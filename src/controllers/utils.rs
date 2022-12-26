use std::error::Error;

use log::{info, warn};
use serde::Deserialize;
use serde_json;

#[derive(Deserialize, Debug)]
struct GRCResponse {
    success: bool,
}

pub async fn validate_captcha(response: String) -> Result<bool, Box<dyn Error>> {
    let secret = match std::env::var("GRC_SECRET") {
        Ok(s) => s,
        Err(err) => {
            warn!("{:?}\nGRC_SECRET not set! returning true.", err);
            return Ok(true);
        }
    };

    let resp = reqwest::get(format!(
        "https://www.google.com/recaptcha/api/siteverify?secret={}&response={}",
        secret, response
    ))
    .await;

    let result: GRCResponse = serde_json::from_str(&resp.unwrap().text().await.unwrap()).unwrap();
    info!("{:?}", result);
    return Ok(result.success);
}
