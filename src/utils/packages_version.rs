use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
struct CrateApiResponse {
    #[serde(rename = "crate")]
    crate_data: CrateInfo,
}

#[derive(Deserialize)]
struct CrateInfo {
    max_stable_version: String,
}

pub async fn get_crate_version(crate_name: &str) -> Result<String, Box<dyn Error>> {
    let url = format!("https://crates.io/api/v1/crates/{}", crate_name);

    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .header(
            reqwest::header::USER_AGENT,
            "GodustCLI/0.1 (https://github.com/viniciusmorgado/godust)",
        )
        .send()
        .await?
        .error_for_status()?;

    let crate_api_response: CrateApiResponse = response.json().await?;

    Ok(crate_api_response.crate_data.max_stable_version)
}
