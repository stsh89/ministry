pub struct ZenMaster<T>
where
    T: SayZenQuote,
{
    assistant: T,
}

pub trait SayZenQuote {
    async fn say_zen_quote(&self) -> Result<String, ReadZenWisdomError>;
}

pub struct Quote {
    pub text: String,
}

pub struct ReadZenWisdomError {
    pub description: String,
}

impl<T> ZenMaster<T>
where
    T: SayZenQuote,
{
    pub fn new(assistant: T) -> Self {
        ZenMaster { assistant }
    }

    pub async fn show_zen_quote(&self) -> Result<Quote, ReadZenWisdomError> {
        let text = self.assistant.say_zen_quote().await?;

        Ok(Quote { text })
    }
}
