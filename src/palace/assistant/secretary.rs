use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{
    datastax::{Datastax, SearchParameters},
    palace::{
        covenant::Idea,
        establishment::student::{DeleteStudentIdea, ListStudentIdeas, WriteIdea},
        MinistryError,
    },
};

pub struct Secretary {
    provider: Datastax,
}

impl Secretary {
    pub fn new(provider: Datastax) -> Self {
        Self { provider }
    }
}

impl WriteIdea for Secretary {
    async fn write_idea(&self, idea: &Idea) -> Result<(), crate::palace::MinistryError> {
        let notebook_idea: NotebookIdea = idea.into();

        let raw_response = self
            .provider
            .read_one("ideas", &notebook_idea.id)
            .await
            .map_err(|err| MinistryError::internal_error(&err.to_string()))?;

        let response: ReadResponse = serde_json::from_str(&raw_response)
            .map_err(|err| MinistryError::internal_error(&err.to_string()))?;

        if response.count > 0 {
            return Err(MinistryError::internal_error("provider data collision"));
        }

        let response = self
            .provider
            .write("ideas", &notebook_idea)
            .await
            .map_err(|err| MinistryError::internal_error(&err.to_string()))?;

        serde_json::from_str::<NotebookIdea>(&response)
            .map_err(|err| MinistryError::internal_error(&err.to_string()))?;

        Ok(())
    }
}

impl ListStudentIdeas for Secretary {
    async fn list_student_ideas(&self, student: &str) -> Result<Vec<Idea>, MinistryError> {
        let raw_response = self
            .provider
            .search(
                "ideas",
                &SearchParameters {
                    r#where: &json!({"student": {"$eq": student}}).to_string(),
                },
            )
            .await
            .map_err(|err| MinistryError::internal_error(&err.to_string()))?;

        let response: ReadResponse = serde_json::from_str(&raw_response)
            .map_err(|err| MinistryError::internal_error(&err.to_string()))?;

        let ideas = response
            .data
            .into_iter()
            .map(|value| value.try_into())
            .collect::<Result<Vec<Idea>, MinistryError>>()?;

        Ok(ideas)
    }
}

impl DeleteStudentIdea for Secretary {
    async fn delete_student_idea(&self, idea: &Idea) -> Result<(), MinistryError> {
        let notebook_ideas = self.list_notebook_ideas(idea.student()).await?;

        let notebook_idea = notebook_ideas
            .into_iter()
            .find(|notebook_idea| notebook_idea.thought == idea.thought())
            .ok_or(MinistryError::covenant_not_found("Idea"))?;

        let result = self
            .provider
            .delete(
                "ideas",
                &[
                    &notebook_idea.student,
                    &notebook_idea.recording_time,
                    &notebook_idea.id,
                ],
            )
            .await
            .map_err(|err| MinistryError::internal_error(&err.to_string()))?;

        if result.is_empty() {
            Ok(())
        } else {
            Err(MinistryError::internal_error(""))
        }
    }
}

impl Secretary {
    async fn list_notebook_ideas(&self, student: &str) -> Result<Vec<NotebookIdea>, MinistryError> {
        let raw_response = self
            .provider
            .search(
                "ideas",
                &SearchParameters {
                    r#where: &json!({"student": {"$eq": student}}).to_string(),
                },
            )
            .await
            .map_err(|err| MinistryError::internal_error(&err.to_string()))?;

        let response: ReadResponse = serde_json::from_str(&raw_response)
            .map_err(|err| MinistryError::internal_error(&err.to_string()))?;

        Ok(response.data)
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct NotebookIdea {
    id: String,
    student: String,
    thought: String,
    recording_time: String,
}

impl TryFrom<NotebookIdea> for Idea {
    type Error = MinistryError;

    fn try_from(value: NotebookIdea) -> Result<Self, Self::Error> {
        Idea::new(&value.thought, &value.student)
    }
}

#[derive(Deserialize)]
struct ReadResponse {
    count: u64,
    data: Vec<NotebookIdea>,
}

impl From<&Idea> for NotebookIdea {
    fn from(value: &Idea) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            thought: value.thought().to_string(),
            student: value.student().to_string(),
            recording_time: Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        }
    }
}
