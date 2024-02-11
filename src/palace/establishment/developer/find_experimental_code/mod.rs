use crate::palace::covenant::CodeExperiment;

use super::list_code_experiments;

pub struct SearchError {
    pub description: String,
}

pub struct CodeExperimentsCatalogue;

impl CodeExperimentsCatalogue {
    pub fn new() -> Self {
        CodeExperimentsCatalogue {}
    }

    pub fn find_code(&self, name: &str) -> Result<CodeExperiment, SearchError> {
        list_code_experiments()
            .into_iter()
            .find(|(code_id, _)| code_id == name)
            .ok_or(SearchError {
                description: format!("Code experiment `{}` not found.", name),
            })
            .map(|(_, code_experiment)| code_experiment)
    }
}
