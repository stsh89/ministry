mod subcommands;

use crate::cli::subcommands::playgrounds::Playgrounds;
use clap::Command;

use self::subcommands::SubcommandNames;

pub struct CliError {
    pub description: String,
}

pub fn run() -> Result<(), CliError> {
    let matches = Command::new("ministry")
        .subcommands(subcommands::Subcommand::list())
        .get_matches();

    execute(matches.subcommand())
}

fn execute(subcommand: Option<(&str, &clap::ArgMatches)>) -> Result<(), CliError> {
    match subcommand {
        Some((name, arg_matches)) => execute_subcommand(name, arg_matches),
        None => Err(CliError {
            description: "missing subcommand name".to_string(),
        }),
    }
}

fn execute_subcommand(name: &str, arg_matches: &clap::ArgMatches) -> Result<(), CliError> {
    use SubcommandNames::*;

    match name.try_into()? {
        PlaygroundSubcommandName => execute_playground_subcommand(arg_matches),
    }
}

fn execute_playground_subcommand(arg_matches: &clap::ArgMatches) -> Result<(), CliError> {
    let ground_name: &str = arg_matches.get_one::<String>("ground").ok_or(CliError {
        description: "missing playground name".to_string(),
    })?;

    TryInto::<Playgrounds>::try_into(ground_name)?.execute();

    Ok(())
}
