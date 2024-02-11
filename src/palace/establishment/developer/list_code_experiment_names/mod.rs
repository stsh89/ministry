pub struct ExperimentsList {}

impl ExperimentsList {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read_names(&self) -> Vec<String> {
        vec!["book".to_string()]
    }
}
