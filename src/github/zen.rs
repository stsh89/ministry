use reqwest::Url;

use super::GithubError;

const BASE_URL: &str = "https://api.github.com";
const USER_AGENT: &str = "ministry";

pub async fn fetch() -> Result<String, GithubError> {
    let resp = reqwest::Client::new()
        .get(zen_url())
        .header("User-Agent", USER_AGENT)
        .send()
        .await
        .map_err(|err| GithubError {
            description: err.to_string(),
        })?
        .text()
        .await
        .map_err(|err| GithubError {
            description: err.to_string(),
        })?;

    Ok(resp)
}

fn base_url() -> Url {
    Url::parse(BASE_URL).unwrap()
}

fn zen_url() -> Url {
    base_url().join("zen").unwrap()
}
