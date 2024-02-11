pub struct ZenWisdom<T>
where
    T: ReadZenWisdom,
{
    assistant: T,
}

pub trait ReadZenWisdom {
    async fn read_zen_wisdom(&self) -> Result<String, ReadZenWisdomError>;
}

pub struct Inscription {
    pub text: String,
}

pub struct ReadZenWisdomError {
    pub description: String,
}

impl<T> ZenWisdom<T>
where
    T: ReadZenWisdom,
{
    pub fn new(pupil: T) -> Self {
        ZenWisdom { assistant: pupil }
    }

    pub async fn read_zen(&self) -> Result<Inscription, ReadZenWisdomError> {
        let text = self.assistant.read_zen_wisdom().await?;

        Ok(Inscription { text })
    }
}
