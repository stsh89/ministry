mod experiment;

pub struct CodeExperiment {
    parameters: Parameters,
}

pub struct Parameters {
    pub experiment_name: ExperimentName,
}

pub enum ExperimentName {
    Book,
}

impl CodeExperiment {
    pub fn new(parameters: Parameters) -> Self {
        Self { parameters }
    }

    pub fn run(&self) {
        use ExperimentName::*;

        match self.parameters.experiment_name {
            Book => run_books_experiment(),
        }
    }
}

fn run_books_experiment() {
    experiment::book::example()
}
