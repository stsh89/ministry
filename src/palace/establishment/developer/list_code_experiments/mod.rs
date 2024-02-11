use crate::palace::covenant::CodeExperiment;

pub struct Laboratory<T>
where
    T: ListCodeExperiments,
{
    manager: T,
}

pub trait ListCodeExperiments {
    fn list_code_experiments(&self) -> Vec<(String, CodeExperiment)>;
}

impl<T> Laboratory<T>
where
    T: ListCodeExperiments,
{
    pub fn new(manager: T) -> Self {
        Self { manager }
    }

    pub fn list_code_experiments(&self) -> Vec<(String, CodeExperiment)> {
        self.manager.list_code_experiments()
    }
}
