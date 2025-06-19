use golem_vector::config::with_config_key;
use golem_vector::golem::vector::types::VectorError;
use log::trace;
use reqwest::{Client, Method, Response};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub struct QdrantClient {
    client: Client,
    base_url: String,
    qdrant_api_key: String,
}

const BASE_URL: &str = "http://localhost:6333";

impl QdrantClient {
    pub fn new() -> Result<Self, VectorError> {
        let base_url = with_config_key(
            "QDRANT_BASE_URL",
            |_| BASE_URL.to_string(),
            |base_url| base_url,
        );
        let qdrant_api_key = with_config_key(
            "QDRANT_API_KEY",
            |err| {
                Err(VectorError::ConnectionError(format!(
                    "Missing Qdrant API Key: {err}"
                )))
            },
            Ok,
        )?;
        let client = Client::builder()
            .build()
            .expect("Failed to initalize HTTP client");
        Ok(Self {
            client,
            base_url,
            qdrant_api_key,
        })
    }
    pub fn connect(&self) -> Result<(), VectorError> {
        trace!("Connecting to Qdrant at {}", self.base_url);
        let url = format!("{}/healthz", self.base_url);
        let response = self
            .client
            .get(url)
            .header("api-Key", &self.qdrant_api_key)
            .send();
        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    trace!("Successfully connected to Qdrant");
                    Ok(())
                } else {
                    Err(VectorError::ConnectionError(format!(
                        "Failed to connect to Qdrant: {}",
                        resp.status()
                    )))
                }
            }
            Err(err) => Err(VectorError::ConnectionError(format!(
                "Error connecting to Qdrant: {}",
                err
            ))),
        }
    }
}
