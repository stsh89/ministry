use std::fmt;

pub struct CliError {
    kind: CliErrorKind,
    description: String,
}

enum CliErrorKind {
    MissingSubcomand(String),
    CommandNotFound(String),
    SubcommandNotFound {
        command_name: String,
        subcommand_name: String,
    },
    PlaygroundNotFound(String),
}

impl CliError {
    pub fn missing_subcommand(name: &str) -> Self {
        Self {
            kind: CliErrorKind::MissingSubcomand(name.to_string()),
            description: "Missing subcommand".to_string(),
        }
    }

    pub fn command_not_found(command_name: &str) -> Self {
        Self {
            kind: CliErrorKind::CommandNotFound(command_name.to_string()),
            description: "Command not found".to_string(),
        }
    }

    pub fn subcommand_not_found(command_name: &str, subcommand_name: &str) -> Self {
        Self {
            kind: CliErrorKind::SubcommandNotFound {
                command_name: command_name.to_string(),
                subcommand_name: subcommand_name.to_string(),
            },
            description: "Subcommand not found".to_string(),
        }
    }

    pub fn playground_not_found(name: &str) -> Self {
        Self {
            kind: CliErrorKind::PlaygroundNotFound(name.to_string()),
            description: "Playground not found".to_string(),
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CliErrorKind::*;

        let description = match &self.kind {
            CommandNotFound(name) => {
                format!("{}. There is no `{}` command.", self.description, name)
            }
            MissingSubcomand(name) => format!(
                "{}. Subcommand is requried for the command `{}`.",
                self.description, name
            ),
            SubcommandNotFound {
                subcommand_name,
                command_name,
            } => format!(
                "{}. There is no subcommand `{}` for `{}` command.",
                self.description, subcommand_name, command_name
            ),
            PlaygroundNotFound(name) => format!(
                "{}. There is no playground with the name `{}`.",
                self.description, name
            ),
        };

        write!(f, "{}", description)
    }
}
