use crate::palace::{
    covenant::simulate_book_to_author_relationship, establishment::developer::ListCodeExperiments,
};

pub struct LaboratoryManager;

impl LaboratoryManager {
    pub fn new() -> Self {
        LaboratoryManager {}
    }
}

impl ListCodeExperiments for LaboratoryManager {
    fn list_code_experiments(&self) -> Vec<(String, crate::palace::covenant::CodeExperiment)> {
        vec![("book".to_string(), simulate_book_to_author_relationship)]
    }
}
