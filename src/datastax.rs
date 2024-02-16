use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Url,
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum DatastaxError {
    #[error("{0}")]
    MalformedUrl(String),

    #[error("{0}")]
    InternalError(String),
}

impl DatastaxError {
    fn malformed_url(url: &str) -> Self {
        Self::MalformedUrl(url.to_string())
    }

    fn internal_error(description: &str) -> Self {
        Self::InternalError(description.to_string())
    }
}

pub struct Datastax {
    client: reqwest::Client,
    base_url: Url,
}

pub struct DatastaxConfig {
    pub database_id: String,
    pub database_region: String,
    pub database_keyspace: String,
    pub database_application_token: String,
}

pub struct SearchParameters<'a> {
    pub r#where: &'a str,
}

impl Datastax {
    pub fn new(config: DatastaxConfig) -> Result<Self, DatastaxError> {
        let builder = reqwest::Client::builder().default_headers(HeaderMap::from_iter(vec![
            (
                reqwest::header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            ),
            (
                reqwest::header::ACCEPT,
                HeaderValue::from_static("application/json"),
            ),
            (
                HeaderName::from_static("x-cassandra-token"),
                HeaderValue::from_str(&config.database_application_token)
                    .map_err(|err| DatastaxError::internal_error(&err.to_string()))?,
            ),
        ]));

        let url_string = &format!(
            "https://{}-{}.apps.astra.datastax.com/api/rest/v2/keyspaces/{}",
            config.database_id, config.database_region, config.database_keyspace
        );

        let base_url =
            Url::parse(url_string).map_err(|err| DatastaxError::malformed_url(&err.to_string()))?;

        builder
            .build()
            .map_err(|err| DatastaxError::internal_error(&err.to_string()))
            .map(|client| Self { client, base_url })
    }

    pub async fn read_one(
        &self,
        database_name: &str,
        primary_key: &str,
    ) -> Result<String, DatastaxError> {
        let mut url = self.base_url.clone();

        url.path_segments_mut()
            .map_err(|_err| DatastaxError::malformed_url(""))?
            .push(database_name)
            .push(primary_key);

        self.client
            .get(url)
            .send()
            .await
            .map_err(|err| DatastaxError::internal_error(&err.to_string()))?
            .text()
            .await
            .map_err(|err| DatastaxError::internal_error(&err.to_string()))
    }

    pub async fn write<T>(&self, database_name: &str, data: &T) -> Result<String, DatastaxError>
    where
        T: Serialize,
    {
        let mut url = self.base_url.clone();

        url.path_segments_mut()
            .map_err(|_err| DatastaxError::malformed_url(""))?
            .push(database_name);

        self.client
            .post(url)
            .json(data)
            .send()
            .await
            .map_err(|err| DatastaxError::internal_error(&err.to_string()))?
            .text()
            .await
            .map_err(|err| DatastaxError::internal_error(&err.to_string()))
    }

    pub async fn search(
        &self,
        database_name: &str,
        search_parameters: &SearchParameters<'_>,
    ) -> Result<String, DatastaxError> {
        let mut url = self.base_url.clone();

        url.path_segments_mut()
            .map_err(|_err| DatastaxError::malformed_url(""))?
            .push(database_name);

        self.client
            .get(url)
            .query(&[("where", search_parameters.r#where)])
            .send()
            .await
            .map_err(|err| DatastaxError::internal_error(&err.to_string()))?
            .text()
            .await
            .map_err(|err| DatastaxError::internal_error(&err.to_string()))
    }

    pub async fn delete(
        &self,
        database_name: &str,
        primary_key: &[&str],
    ) -> Result<String, DatastaxError> {
        let mut url = self.base_url.clone();

        url.path_segments_mut()
            .map_err(|_err| DatastaxError::malformed_url(""))?
            .push(database_name);

        for segment in primary_key {
            url.path_segments_mut()
                .map_err(|_err| DatastaxError::malformed_url(""))?
                .push(segment);
        }

        self.client
            .delete(url)
            .send()
            .await
            .map_err(|err| DatastaxError::internal_error(&err.to_string()))?
            .text()
            .await
            .map_err(|err| DatastaxError::internal_error(&err.to_string()))
    }
}
