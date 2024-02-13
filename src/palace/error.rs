#[derive(Debug, thiserror::Error)]
pub enum MinistryError {
    #[error("Covenant: {covenant}. Violation: {violation}")]
    ViolatedCovenant { covenant: String, violation: String },
}

impl MinistryError {
    pub fn covenant_violation(covenant: impl ToString, violation: impl ToString) -> Self {
        MinistryError::ViolatedCovenant {
            covenant: covenant.to_string(),
            violation: violation.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_violated_covenantn() {
        let error = MinistryError::covenant_violation(
            "Code experiment",
            "should not cause any harm to the environment.",
        );

        assert_eq!(
            error.to_string(),
            "Covenant: Code experiment. Violation: should not cause any harm to the environment."
        );
    }
}
