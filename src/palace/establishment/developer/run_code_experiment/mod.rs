use super::find_code_experiment_by_id;

pub struct Laboratory;

pub struct InvocationError {
    pub description: String,
}

impl Laboratory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run_experiment(&self, name: &str) -> Result<(), InvocationError> {
        let experiment = find_code_experiment_by_id(name).map_err(|err| InvocationError {
            description: err.description,
        })?;

        experiment();

        Ok(())
    }
}
