pub mod playgrounds;

use super::CliError;

pub enum SubcommandNames {
    PlaygroundSubcommandName,
}

pub struct Subcommand {
    description: String,
    name: String,
    args: Vec<SubcommandArgs>,
}

pub struct SubcommandArgs {
    name: String,
    description: String,
}

impl From<SubcommandNames> for String {
    fn from(value: SubcommandNames) -> Self {
        use SubcommandNames::*;

        match value {
            PlaygroundSubcommandName => "playground",
        }
        .to_string()
    }
}

impl TryFrom<&str> for SubcommandNames {
    type Error = CliError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use SubcommandNames::*;

        let subcommand = match value {
            "playground" => PlaygroundSubcommandName,
            name => {
                return Err(CliError {
                    description: format!("not registered subcommand name: {name}"),
                })
            }
        };

        Ok(subcommand)
    }
}

impl Subcommand {
    pub fn list() -> Vec<Subcommand> {
        use SubcommandNames::*;

        vec![Subcommand {
            description: "Run experimental code".to_string(),
            name: PlaygroundSubcommandName.into(),
            args: vec![SubcommandArgs {
                name: "ground".to_string(),
                description: "Required name of the ground to use".to_string(),
            }],
        }]
    }
}

impl From<Subcommand> for clap::Command {
    fn from(value: Subcommand) -> Self {
        clap::Command::new(value.name)
            .about(value.description)
            .args(value.args)
    }
}

impl From<SubcommandArgs> for clap::Arg {
    fn from(value: SubcommandArgs) -> Self {
        clap::Arg::new(value.name).help(value.description)
    }
}
