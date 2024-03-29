#[derive(Debug, thiserror::Error)]
pub enum MinistryError {
    #[error("{0} not found")]
    CovenantNotFound(String),

    #[error("Covenant: {covenant}. Violation: {violation}")]
    ViolatedCovenant { covenant: String, violation: String },

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl MinistryError {
    pub fn covenant_violation(covenant: impl ToString, violation: impl ToString) -> Self {
        Self::ViolatedCovenant {
            covenant: covenant.to_string(),
            violation: violation.to_string(),
        }
    }

    pub fn internal_error(description: &str) -> Self {
        Self::InternalError(description.to_string())
    }

    pub fn covenant_not_found(covenant: &str) -> Self {
        Self::CovenantNotFound(covenant.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_violated_covenant() {
        let error = MinistryError::covenant_violation(
            "Code experiment",
            "should not cause any harm to the environment.",
        );

        assert_eq!(
            error.to_string(),
            "Covenant: Code experiment. Violation: should not cause any harm to the environment."
        );
    }

    #[test]
    fn test_internal_error() {
        let error = MinistryError::internal_error("Something goes wrong");

        assert_eq!(error.to_string(), "Internal error: Something goes wrong");
    }
}
