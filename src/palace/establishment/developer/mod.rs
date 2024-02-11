//! Technical director of the Ministry.
//! More generally, it is just one of the actors in the application..

mod find_experimental_code;
mod list_code_experiments;
mod read_zen_wisdom;
mod run_code_experiment;

pub use list_code_experiments::ListCodeExperiments;
pub use read_zen_wisdom::{ReadZenWisdomError, SayZenQuote};

use crate::palace::{
    assistant::{LaboratoryManager, Psychologist},
    covenant::CodeExperiment,
};

pub fn run_code_experiment(
    experiment_name: &str,
) -> Result<(), run_code_experiment::InvocationError> {
    use run_code_experiment::*;

    let code_experiment = Laboratory::new();

    code_experiment.run_experiment(experiment_name)
}

pub async fn show_zen_quote() -> Result<read_zen_wisdom::Quote, read_zen_wisdom::ReadZenWisdomError>
{
    use read_zen_wisdom::*;

    let zen_wisdom = ZenMaster::new(Psychologist::new());

    zen_wisdom.show_zen_quote().await
}

pub fn list_code_experiments() -> Vec<(String, CodeExperiment)> {
    use list_code_experiments::*;

    Laboratory::new(LaboratoryManager::new()).list_code_experiments()
}

pub fn find_code_experiment_by_id(
    experiment_name: &str,
) -> Result<fn() -> (), find_experimental_code::SearchError> {
    use find_experimental_code::*;

    let catalogoue = CodeExperimentsCatalogue::new();

    catalogoue.find_code(experiment_name)
}
