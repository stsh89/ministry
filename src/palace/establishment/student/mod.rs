use crate::palace::{covenant::Idea, MinistryError};

pub trait WriteIdea {
    async fn write_idea(&self, idea: &Idea) -> Result<(), MinistryError>;
}

pub trait ListStudentIdeas {
    async fn list_student_ideas(&self, student: &str) -> Result<Vec<Idea>, MinistryError>;
}

pub trait DeleteStudentIdea {
    async fn delete_student_idea(&self, idea: &Idea) -> Result<(), MinistryError>;
}

pub struct NewIdeaParameters<'a, T>
where
    T: WriteIdea,
{
    pub secretary: T,
    pub thought: &'a str,
    pub student: &'a str,
}

pub struct ListIdeasParameters<'a, T>
where
    T: ListStudentIdeas,
{
    pub secretary: T,
    pub student: &'a str,
}

pub struct DeleteIdeaParameters<'a, T>
where
    T: DeleteStudentIdea,
{
    pub secretary: T,
    pub idea: &'a Idea,
}

pub async fn note_new_idea<T>(parameters: NewIdeaParameters<'_, T>) -> Result<Idea, MinistryError>
where
    T: WriteIdea,
{
    let idea = Idea::new(parameters.thought, parameters.student)?;

    parameters.secretary.write_idea(&idea).await?;

    Ok(idea)
}

pub async fn list_ideas<T>(
    parameters: ListIdeasParameters<'_, T>,
) -> Result<Vec<Idea>, MinistryError>
where
    T: ListStudentIdeas,
{
    parameters
        .secretary
        .list_student_ideas(parameters.student)
        .await
}

pub async fn delete_idea<T>(parameters: DeleteIdeaParameters<'_, T>) -> Result<(), MinistryError>
where
    T: DeleteStudentIdea,
{
    parameters
        .secretary
        .delete_student_idea(parameters.idea)
        .await
}
