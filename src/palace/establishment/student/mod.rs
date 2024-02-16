use crate::palace::{covenant::Idea, MinistryError};

pub trait WriteIdea {
    async fn write_idea(&self, idea: &Idea) -> Result<(), MinistryError>;
}

pub trait ListStudentIdeas {
    async fn list_student_ideas(&self, student: &str) -> Result<Vec<Idea>, MinistryError>;
}

pub struct NewIdeaParameters<'a, T>
where
    T: WriteIdea,
{
    pub secretary: T,
    pub thought: &'a str,
    pub student: &'a str,
}

pub struct ListIdeas<'a, T>
where
    T: ListStudentIdeas,
{
    pub secretary: T,
    pub student: &'a str,
}

pub async fn note_new_idea<T>(parameters: NewIdeaParameters<'_, T>) -> Result<Idea, MinistryError>
where
    T: WriteIdea,
{
    let idea = Idea::new(parameters.thought, parameters.student)?;

    parameters.secretary.write_idea(&idea).await?;

    Ok(idea)
}

pub async fn list_ideas<T>(parameters: ListIdeas<'_, T>) -> Result<Vec<Idea>, MinistryError>
where
    T: ListStudentIdeas,
{
    parameters
        .secretary
        .list_student_ideas(parameters.student)
        .await
}
