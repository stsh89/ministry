use crate::palace::MinistryError;

pub struct Idea {
    thought: String,
    student: String,
}

impl Idea {
    pub fn new(thought: &str, student: &str) -> Result<Self, MinistryError> {
        ThoughtValidator::new(thought).validate()?;

        Ok(Self {
            thought: thought.to_string(),
            student: student.to_string(),
        })
    }

    pub fn thought(&self) -> &str {
        &self.thought
    }

    pub fn student(&self) -> &str {
        &self.student
    }
}

struct ThoughtValidator<'a> {
    thought: &'a str,
}

impl<'a> ThoughtValidator<'a> {
    fn new(thought: &'a str) -> Self {
        Self { thought }
    }

    fn validate(self) -> Result<(), MinistryError> {
        self.validate_presence()?.validate_length()?;

        Ok(())
    }

    fn validate_presence(self) -> Result<Self, MinistryError> {
        if self.thought.is_empty() {
            Err(MinistryError::covenant_violation(
                "Idea",
                "Thought can not be empty",
            ))
        } else {
            Ok(self)
        }
    }

    fn validate_length(self) -> Result<Self, MinistryError> {
        if self.thought.len() > 500 {
            Err(MinistryError::covenant_violation(
                "Idea",
                "Thought can not exceed the limit of 500 bytes",
            ))
        } else {
            Ok(self)
        }
    }
}
