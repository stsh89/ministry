//! Technical director of the Ministry.
//! More generally, it is just one of the actors in the application..

mod list_code_experiment_names;
mod read_zen_wisdom;
mod run_code_experiment;

pub use read_zen_wisdom::{ReadZenWisdom, ReadZenWisdomError};
pub use run_code_experiment::{ExperimentName, Parameters as CodeExperimentParameters};

use crate::palace::assistant::ZenWisdomAssistant;

pub fn run_code_experiment(parameters: run_code_experiment::Parameters) {
    use run_code_experiment::*;

    let code_experiment = CodeExperiment::new(parameters);

    code_experiment.run()
}

pub async fn read_zen_wisdom(
) -> Result<read_zen_wisdom::Inscription, read_zen_wisdom::ReadZenWisdomError> {
    use read_zen_wisdom::*;

    let github = ZenWisdom::new(ZenWisdomAssistant::new());

    github.read_zen().await
}

pub fn list_code_experiment_names() -> Vec<String> {
    use list_code_experiment_names::*;

    ExperimentsList::new().read_names()
}
