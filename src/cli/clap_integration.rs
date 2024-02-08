use clap::{Arg, ArgMatches, Command};

use super::{error::CliError, Cli};

pub fn get_command() -> Result<Cli, CliError> {
    let command = command();
    let arg_matches = command.get_matches();
    let subcommand = arg_matches.subcommand();

    let Some((subcommand, arg_matches)) = subcommand else {
        return Err(CliError { description: "Missing subcommand".to_string() });
    };

    match subcommand {
        "playground" => get_playground_command(arg_matches),
        _ => Err(CliError {
            description: "Subcommand not found".to_string(),
        }),
    }
}

fn get_playground_command(arg_matches: &ArgMatches) -> Result<Cli, CliError> {
    let subcommand = arg_matches.subcommand();

    let Some((subcommand, arg_matches)) = subcommand else {
        return Err(CliError { description: "Missing subcommand".to_string() });
    };

    let cli = match subcommand {
        "list" => get_playground_list_command(),
        "run" => get_playground_run_command(arg_matches)?,
        _ => {
            return Err(CliError {
                description: "Subcommand not found".to_string(),
            })
        }
    };

    Ok(cli)
}

fn get_playground_list_command() -> Cli {
    Cli::playground_list_command()
}

fn get_playground_run_command(arg_matches: &ArgMatches) -> Result<Cli, CliError> {
    let empty_playground_name = String::new();
    let name = arg_matches
        .get_one::<String>("name")
        .unwrap_or(&empty_playground_name);

    let cli = Cli::playground_run_command(name)?;

    Ok(cli)
}

fn command() -> clap::Command {
    clap::Command::new("ministry")
        .subcommand(playground_command())
        .subcommand_required(true)
}

pub fn playground_command() -> Command {
    Command::new("playground")
        .about("Platform for running experimental code")
        .subcommand_required(true)
        .subcommand(playground_list_command())
        .subcommand(playground_run_command())
}

fn playground_list_command() -> Command {
    Command::new("list").about("List names of all playgrounds")
}

fn playground_run_command() -> Command {
    let name_arg = Arg::new("name")
        .required(true)
        .help("The name of the playground to run");

    Command::new("run")
        .about("Run playground code")
        .arg(name_arg)
}
