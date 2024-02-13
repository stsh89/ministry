use crate::palace::{covenant::Idea, MinistryError};

pub fn note_new_idea(thought: &str) -> Result<Idea, MinistryError> {
    Idea::new(thought)
}
