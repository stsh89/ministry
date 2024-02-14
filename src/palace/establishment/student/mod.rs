use crate::palace::{covenant::Idea, MinistryError};

pub trait WriteIdea {
    async fn write_idea(&self, idea: &Idea) -> Result<(), MinistryError>;
}

pub struct NewIdeaParameters<'a, T>
where
    T: WriteIdea,
{
    pub notebook: T,
    pub thought: &'a str,
}

pub async fn note_new_idea<T>(parameters: NewIdeaParameters<'_, T>) -> Result<Idea, MinistryError>
where
    T: WriteIdea,
{
    let idea = Idea::new(parameters.thought)?;

    parameters.notebook.write_idea(&idea).await?;

    Ok(idea)
}
