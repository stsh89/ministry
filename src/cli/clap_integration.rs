use clap::{Arg, Command};

use super::{error::CliError, playground};

pub fn run() -> Result<(), CliError> {
    let command = command();

    let arg_matches = command.get_matches();

    let subcommand = arg_matches.subcommand();

    let Some((subcommand, arg_matches)) = subcommand else {
        return Err(CliError { description: "Missing subcommand".to_string() });
    };

    match subcommand {
        "playground" => {
            let subcommand = arg_matches.subcommand();

            let Some((subcommand, arg_matches)) = subcommand else {
                return Err(CliError { description: "Missing subcommand".to_string() });
            };

            match subcommand {
                "list" => playground::ListCli::new().run(),
                "run" => {
                    let name = arg_matches
                        .get_one::<String>("name")
                        .map(|name| name.to_string())
                        .unwrap_or_default();
                    playground::RunCli::new(&name)?.run()
                }
                _ => {
                    return Err(CliError {
                        description: "Subcommand not found".to_string(),
                    })
                }
            }
        }
        _ => {
            return Err(CliError {
                description: "Subcommand not found".to_string(),
            })
        }
    }

    Ok(())
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
        .long("name")
        .required(true)
        .help("The name of the playground to run");

    Command::new("run")
        .about("Run playground code")
        .arg(name_arg)
}
