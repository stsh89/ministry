use reqwest::Url;

struct Github {
    client: reqwest::Client,
    base_url: Url,
}

pub struct GithubError {
    pub description: String,
}

impl Github {
    pub fn new() -> Result<Self, GithubError> {
        let builder = reqwest::Client::builder().user_agent("ministry");
        let base_url = Url::parse("https://api.github.com").map_err(|err| GithubError {
            description: err.to_string(),
        })?;

        builder
            .build()
            .map_err(|err| GithubError {
                description: err.to_string(),
            })
            .map(|client| Self { client, base_url })
    }

    pub async fn zen(&self) -> Result<String, GithubError> {
        let url = self.base_url.join("zen").map_err(|err| GithubError {
            description: err.to_string(),
        })?;

        self.client
            .get(url)
            .send()
            .await
            .map_err(|err| GithubError {
                description: err.to_string(),
            })?
            .text()
            .await
            .map_err(|err| GithubError {
                description: err.to_string(),
            })
    }
}

pub async fn zen() -> Result<String, GithubError> {
    Github::new()?.zen().await
}
