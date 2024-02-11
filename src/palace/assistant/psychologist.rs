use crate::github;
use crate::palace::establishment::developer::{ReadZenWisdomError, SayZenQuote};

pub struct Psychologist;

impl Psychologist {
    pub fn new() -> Self {
        Self
    }
}

impl SayZenQuote for Psychologist {
    async fn say_zen_quote(&self) -> Result<String, ReadZenWisdomError> {
        github::zen().await.map_err(|_err| ReadZenWisdomError {
            description: "Zen Master is not available. Please try again later".to_string(),
        })
    }
}
