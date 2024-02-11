use crate::github;
use crate::palace::establishment::developer::{ReadZenWisdom, ReadZenWisdomError};

pub struct ZenWisdomAssistant;

impl ZenWisdomAssistant {
    pub fn new() -> Self {
        Self
    }
}

impl ReadZenWisdom for ZenWisdomAssistant {
    async fn read_zen_wisdom(&self) -> Result<String, ReadZenWisdomError> {
        github::zen().await.map_err(|_err| ReadZenWisdomError {
            description: "Zen Master is not available. Please try again later".to_string(),
        })
    }
}
